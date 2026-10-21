//! Rust-native Ant Design form layout and deterministic field validation.
//!
//! Form values and validation rules are plain Rust, independent of the window.
//! `AntForm` and `AntFormItem` draw GPUI controls; applications own field
//! state and send values to `FormSchema::validate` on change or submission.

use crate::AntTheme;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _,
    Window, div, px,
};
use std::collections::BTreeMap;

/// Simple, deterministic validation rules; optional empty fields do not
/// trigger rules other than Required.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Email,
    EqualsField(String),
}

#[derive(Clone, Debug)]
pub struct FormFieldSchema {
    pub name: String,
    pub label: String,
    pub rules: Vec<FormRule>,
}

/// Declarative validation schema for a group of native GPUI inputs.
#[derive(Clone, Debug, Default)]
pub struct FormSchema {
    fields: Vec<FormFieldSchema>,
}

/// Errors are keyed by the developer's stable field name. The first failed
/// rule is shown for each field, in its declared rule order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FormValidation {
    pub errors: BTreeMap<String, String>,
}

impl FormValidation {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
    pub fn message(&self, field: &str) -> Option<&str> {
        self.errors.get(field).map(String::as_str)
    }
}

/// Conservative UI-friendly email check (not a complete RFC 5322 parser).
fn email_shape_valid(input: &str) -> bool {
    if input.bytes().any(|c| c.is_ascii_whitespace()) {
        return false;
    }
    let Some((local, domain)) = input.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() || domain.contains('@') {
        return false;
    }
    let Some((host, suffix)) = domain.rsplit_once('.') else {
        return false;
    };
    !host.is_empty() && suffix.len() >= 2 && !suffix.starts_with('-')
}

impl FormSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field(
        mut self,
        name: impl Into<String>,
        label: impl Into<String>,
        rules: impl IntoIterator<Item = FormRule>,
    ) -> Self {
        self.fields.push(FormFieldSchema {
            name: name.into(),
            label: label.into(),
            rules: rules.into_iter().collect(),
        });
        self
    }

    pub fn validate(&self, values: &BTreeMap<String, String>) -> FormValidation {
        let mut errors = BTreeMap::new();
        for field in &self.fields {
            let value = values
                .get(&field.name)
                .map(String::as_str)
                .unwrap_or("")
                .trim();
            for rule in &field.rules {
                let error = match rule {
                    FormRule::Required if value.is_empty() => {
                        Some(format!("Please enter {}", field.label))
                    }
                    FormRule::MinLength(min)
                        if !value.is_empty() && value.chars().count() < *min =>
                    {
                        Some(format!(
                            "{} must contain at least {} characters",
                            field.label, min
                        ))
                    }
                    FormRule::MaxLength(max) if value.chars().count() > *max => Some(format!(
                        "{} must contain at most {} characters",
                        field.label, max
                    )),
                    FormRule::Email if !value.is_empty() && !email_shape_valid(value) => {
                        Some(format!("Enter a valid {}", field.label))
                    }
                    FormRule::EqualsField(other)
                        if !value.is_empty()
                            && value != values.get(other).map(|s| s.trim()).unwrap_or("") =>
                    {
                        Some(format!("{} does not match {}", field.label, other))
                    }
                    _ => None,
                };
                if let Some(error) = error {
                    errors.insert(field.name.clone(), error);
                    break;
                }
            }
        }
        FormValidation { errors }
    }
}

/// Layout-only native form. Field validation and submission are application
/// owned rather than hidden in GPUI element state.
#[derive(IntoElement)]
pub struct AntForm {
    children: Vec<AnyElement>,
    gap: f32,
    theme: AntTheme,
}
impl Default for AntForm {
    fn default() -> Self {
        Self::new()
    }
}
impl AntForm {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 18.0,
            theme: AntTheme::default(),
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn gap(mut self, value: f32) -> Self {
        self.gap = value.max(0.);
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
impl RenderOnce for AntForm {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(self.gap))
            .text_color(self.theme.text)
            .children(self.children)
    }
}

/// A label, a GPUI input, help text and an optional inline error.
/// For screen readers supply the label to the control's `aria_label` API.
#[derive(IntoElement)]
pub struct AntFormItem {
    label: SharedString,
    required: bool,
    help: Option<SharedString>,
    error: Option<SharedString>,
    children: Vec<AnyElement>,
    theme: AntTheme,
}
impl AntFormItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            required: false,
            help: None,
            error: None,
            children: Vec::new(),
            theme: AntTheme::default(),
        }
    }
    pub fn theme(mut self, theme: AntTheme) -> Self {
        self.theme = theme;
        self
    }
    pub fn required(mut self, yes: bool) -> Self {
        self.required = yes;
        self
    }
    pub fn help(mut self, help: impl Into<SharedString>) -> Self {
        self.help = Some(help.into());
        self
    }
    pub fn error(mut self, error: Option<SharedString>) -> Self {
        self.error = error;
        self
    }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}
impl RenderOnce for AntFormItem {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let t = self.theme;
        let label = div()
            .flex()
            .items_center()
            .gap(px(4.))
            .text_color(t.text)
            .text_size(px(14.))
            .child(self.label)
            .when(self.required, |this| {
                this.child(div().text_color(t.error).child("*"))
            });
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(7.))
            .child(label)
            .children(self.children)
            .when_some(self.error, |this, error| {
                this.child(div().text_size(px(12.)).text_color(t.error).child(error))
            })
            .when_some(self.help, |this, help| {
                this.child(
                    div()
                        .text_size(px(12.))
                        .text_color(t.text_secondary)
                        .child(help),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
        entries
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn required_is_checked_even_when_field_missing() {
        let schema = FormSchema::new().field("name", "name", [FormRule::Required]);
        assert_eq!(
            schema.validate(&values(&[])).message("name"),
            Some("Please enter name")
        );
        assert_eq!(
            schema.validate(&values(&[("name", "   ")])).is_valid(),
            false
        );
        assert!(schema.validate(&values(&[("name", "Jane")])).is_valid());
    }

    #[test]
    fn unicode_lengths_count_characters_not_bytes() {
        let schema = FormSchema::new().field("name", "Name", [FormRule::MinLength(2)]);
        assert!(schema.validate(&values(&[("name", "中文")])).is_valid());
        assert!(!schema.validate(&values(&[("name", "中")])).is_valid());
    }

    #[test]
    fn empty_optional_fields_skip_email_and_min_length() {
        let schema = FormSchema::new()
            .field("email", "Email", [FormRule::Email])
            .field("name", "Name", [FormRule::MinLength(3)]);
        assert!(schema.validate(&values(&[])).is_valid());
    }

    #[test]
    fn email_format_is_conservative() {
        let schema =
            FormSchema::new().field("email", "Email", [FormRule::Required, FormRule::Email]);
        assert!(
            schema
                .validate(&values(&[("email", "hello@example.com")]))
                .is_valid()
        );
        for invalid in [
            "hello",
            "x@",
            "a@@example.com",
            "x@host",
            "x@.com",
            "x@foo.c",
        ] {
            assert!(
                !schema.validate(&values(&[("email", invalid)])).is_valid(),
                "{}",
                invalid
            );
        }
    }

    #[test]
    fn field_equality_and_maximum_length() {
        let schema = FormSchema::new()
            .field("password", "Password", [FormRule::MaxLength(8)])
            .field(
                "confirmation",
                "Confirmation",
                [FormRule::Required, FormRule::EqualsField("password".into())],
            );
        assert!(
            schema
                .validate(&values(&[
                    ("password", "welcome"),
                    ("confirmation", "welcome")
                ]))
                .is_valid()
        );
        assert!(
            !schema
                .validate(&values(&[
                    ("password", "longpassword"),
                    ("confirmation", "wrong")
                ]))
                .is_valid()
        );
    }

    #[test]
    fn first_failing_rule_wins() {
        let schema =
            FormSchema::new().field("name", "Name", [FormRule::Required, FormRule::MinLength(4)]);
        assert_eq!(
            schema.validate(&values(&[])).message("name"),
            Some("Please enter Name")
        );
        assert_eq!(
            schema.validate(&values(&[("name", "Hi")])).message("name"),
            Some("Name must contain at least 4 characters")
        );
    }
}
