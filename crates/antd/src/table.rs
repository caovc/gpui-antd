//! Small to medium Ant Design-style Table using the native GPUI Kit Table
//! semantics. Column sort, exact-match filters, paged rows and row selection are controlled.
//!
//! This is NOT the high-volume virtualized DataTable: there is no fixed
//! header/column, resizable columns, advanced expression filters, tree row or async source.
//! Use GPUI Kit DataTable or the independent gpui-grid for large datasets.
use std::{collections::{BTreeMap,BTreeSet},rc::Rc};
use gpui_kit::{
    App, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled as _, Window, div, px,
};
use gpui_kit::base::StyledExt as _;
use gpui_kit::component::table::{
    Table as KitTable, TableHeader, TableBody, TableRow as KitRow,
    TableHead, TableCell,
};
use gpui_kit::prelude::FluentBuilder as _;
use crate::{AntButton,AntCheckbox,AntDropdown,AntDropdownItem,AntPagination,AntTheme,ControlSize};
use crate::design_tokens::AntComponent;

#[derive(Debug,Clone)]
pub struct AntTableColumn{
    key:SharedString,
    title:SharedString,
    width:Option<f32>,
    sortable:bool,
    numeric:bool,
    filters:Vec<SharedString>,
}
impl AntTableColumn{
    pub fn new(key:impl Into<SharedString>,title:impl Into<SharedString>)->Self{
        Self{key:key.into(),title:title.into(),width:None,sortable:false,numeric:false,filters:Vec::new()}
    }
    pub fn width(mut self,width:f32)->Self{
        self.width=Some(if width.is_finite(){width.max(100.)}else{160.});self
    }
    pub fn sortable(mut self,yes:bool)->Self{self.sortable=yes;self}
    pub fn numeric(mut self,yes:bool)->Self{self.numeric=yes;self}
    /// Exact-match filter menu choices for this column. The active choice
    /// is application-controlled through AntTable::active_filters.
    pub fn filters<I,S>(mut self,values:I)->Self
    where I:IntoIterator<Item=S>,S:Into<SharedString>{
        self.filters=values.into_iter().map(Into::into).collect();self
    }
}
#[derive(Debug,Clone)]
pub struct AntTableRow {
    key:SharedString,
    cells:Vec<SharedString>,
    disabled:bool,
}
impl AntTableRow{
    pub fn new(key:impl Into<SharedString>)->Self{
        Self{key:key.into(),cells:Vec::new(),disabled:false}
    }
    pub fn cell(mut self,value:impl Into<SharedString>)->Self{
        self.cells.push(value.into());self
    }
    pub fn cells<I,S>(mut self,values:I)->Self where I:IntoIterator<Item=S>,S:Into<SharedString>{
        self.cells.extend(values.into_iter().map(Into::into));self
    }
    pub fn disabled(mut self,value:bool)->Self{self.disabled=value;self}
}

/// Three-state Ant Design sort cycle. Sort requests never mutate the model.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub enum TableSort {
    #[default] None,
    Ascending,
    Descending,
}
impl TableSort{
    pub fn next(self)->Self{
        match self{
            Self::None=>Self::Ascending,
            Self::Ascending=>Self::Descending,
            Self::Descending=>Self::None,
        }
    }
    pub fn mark(self)->&'static str{
        match self{Self::None=>"↕",Self::Ascending=>"↑",Self::Descending=>"↓"}
    }
}

/// Sort only by an explicitly sortable column. Numeric columns compare finite
/// values numerically and fall back to case-insensitive text comparison.
pub fn sorted_table_indices(
    columns:&[AntTableColumn],
    rows:&[AntTableRow],
    column:Option<&str>,
    direction:TableSort,
)->Vec<usize>{
    let mut out:Vec<usize>=(0..rows.len()).collect();
    if direction==TableSort::None{return out;}
    let Some(column_ix)=column.and_then(|key|
        columns.iter().position(|col|col.key.as_ref()==key&&col.sortable)
    )else{return out;};
    let numeric=columns[column_ix].numeric;
    out.sort_by(|a,b|{
        let x=rows[*a].cells.get(column_ix).map(|v|v.as_ref()).unwrap_or("");
        let y=rows[*b].cells.get(column_ix).map(|v|v.as_ref()).unwrap_or("");
        let cmp=if numeric{
            match (x.parse::<f64>(),y.parse::<f64>()){
                (Ok(left),Ok(right)) if left.is_finite()&&right.is_finite()=>
                    left.total_cmp(&right),
                _=>x.to_lowercase().cmp(&y.to_lowercase()),
            }
        }else{x.to_lowercase().cmp(&y.to_lowercase())};
        if direction==TableSort::Descending{cmp.reverse()}else{cmp}
    });
    out
}


/// Exact-match filter model. Unknown columns or columns without configured
/// filter choices have no effect. Rows without a filtered cell do not match.
pub fn filtered_table_indices(
    columns:&[AntTableColumn],rows:&[AntTableRow],
    filters:&BTreeMap<String,String>,
)->Vec<usize>{
    let constraints:Vec<(usize,&str)>=filters.iter().filter_map(|(key,value)|{
        columns.iter().enumerate().find(|(_,col)|
            col.key.as_ref()==key.as_str()&&!col.filters.is_empty()
                &&col.filters.iter().any(|allowed|allowed.as_ref()==value.as_str()))
            .map(|(index,_)|(index,value.as_str()))
    }).collect();
    rows.iter().enumerate().filter_map(|(index,row)|{
        constraints.iter().all(|(col,value)|
            row.cells.get(*col).is_some_and(|cell|cell.as_ref()==*value))
            .then_some(index)
    }).collect()
}

/// Filtering precedes sorting and pagination while maintaining stable row
/// identity for controlled selections across filter or page changes.
pub fn table_view_indices(
    columns:&[AntTableColumn],rows:&[AntTableRow],
    filters:&BTreeMap<String,String>,
    sort_column:Option<&str>,direction:TableSort,
)->Vec<usize>{
    let matching=filtered_table_indices(columns,rows,filters);
    let mut order=sorted_table_indices(columns,rows,sort_column,direction);
    order.retain(|index|matching.binary_search(index).is_ok());
    order
}

/// Apply a select-all action only to eligible visible row keys. Existing
/// selections on other pages or hidden by filters are preserved.
pub fn toggle_table_visible_selection(
    visible_keys:&[String],selected:&BTreeSet<String>,next:bool
)->BTreeSet<String>{
    let mut updated=selected.clone();
    for key in visible_keys{
        if next{updated.insert(key.clone());}
        else{updated.remove(key);}
    }
    updated
}

type SortHandler=Rc<dyn Fn(SharedString,TableSort,&mut Window,&mut App)>;
type SelectionHandler=Rc<dyn Fn(BTreeSet<String>,&mut Window,&mut App)>;
type PageHandler=Rc<dyn Fn(usize,&mut Window,&mut App)>;
type FilterHandler=Rc<dyn Fn(SharedString,Option<SharedString>,&mut Window,&mut App)>;

/// Controlled table with native table semantics, sortable column headers,
/// checkbox row selection and a real AntPagination footer.
#[derive(IntoElement)]
pub struct AntTable {
    id:ElementId,
    columns:Vec<AntTableColumn>,
    rows:Vec<AntTableRow>,
    sort_key:Option<SharedString>,
    sort:TableSort,
    selected_keys:BTreeSet<String>,
    active_filters:BTreeMap<String,String>,
    selectable:bool,
    striped:bool,
    bordered:bool,
    pagination:bool,
    page:usize,
    page_size:usize,
    theme:AntTheme,
    on_sort:Option<SortHandler>,
    on_selection:Option<SelectionHandler>,
    on_page_change:Option<PageHandler>,
    on_filter_change:Option<FilterHandler>,
}
impl AntTable {
    pub fn new(id:impl Into<ElementId>)->Self{
        Self{id:id.into(),columns:Vec::new(),rows:Vec::new(),
            sort_key:None,sort:TableSort::None,selected_keys:BTreeSet::new(),active_filters:BTreeMap::new(),
            selectable:false,striped:false,bordered:true,pagination:true,
            page:1,page_size:10,theme:AntTheme::default(),
            on_sort:None,on_selection:None,on_page_change:None,on_filter_change:None}
    }
    pub fn column(mut self,column:AntTableColumn)->Self{self.columns.push(column);self}
    pub fn row(mut self,row:AntTableRow)->Self{self.rows.push(row);self}
    pub fn rows(mut self,rows:impl IntoIterator<Item=AntTableRow>)->Self{
        self.rows.extend(rows);self
    }
    pub fn sort_by(mut self,key:impl Into<SharedString>,direction:TableSort)->Self{
        self.sort_key=Some(key.into());self.sort=direction;self
    }
    pub fn selected_keys<I,S>(mut self,keys:I)->Self where I:IntoIterator<Item=S>,S:AsRef<str>{
        self.selected_keys=keys.into_iter().map(|key|key.as_ref().to_owned()).collect();self
    }
    /// Controlled exact-match values, one selected value per column.
    pub fn active_filters<I,K,V>(mut self,filters:I)->Self
    where I:IntoIterator<Item=(K,V)>,K:AsRef<str>,V:AsRef<str>{
        self.active_filters=filters.into_iter()
            .map(|(key,value)|(key.as_ref().to_owned(),value.as_ref().to_owned())).collect();
        self
    }
    pub fn selectable(mut self,yes:bool)->Self{self.selectable=yes;self}
    pub fn striped(mut self,yes:bool)->Self{self.striped=yes;self}
    pub fn bordered(mut self,yes:bool)->Self{self.bordered=yes;self}
    pub fn pagination(mut self,yes:bool)->Self{self.pagination=yes;self}
    pub fn page(mut self,page:usize)->Self{self.page=page;self}
    pub fn page_size(mut self,size:usize)->Self{self.page_size=size.max(1);self}
    pub fn theme(mut self,theme:AntTheme)->Self{self.theme=theme;self}
    pub fn on_sort(mut self,f:impl Fn(SharedString,TableSort,&mut Window,&mut App)+'static)->Self{
        self.on_sort=Some(Rc::new(f));self
    }
    pub fn on_selection(mut self,f:impl Fn(BTreeSet<String>,&mut Window,&mut App)+'static)->Self{
        self.on_selection=Some(Rc::new(f));self
    }
    pub fn on_page_change(mut self,f:impl Fn(usize,&mut Window,&mut App)+'static)->Self{
        self.on_page_change=Some(Rc::new(f));self
    }
    pub fn on_filter_change(
        mut self,handler:impl Fn(SharedString,Option<SharedString>,&mut Window,&mut App)+'static
    )->Self{
        self.on_filter_change=Some(Rc::new(handler));self
    }
    pub fn page_count(&self)->usize{
        crate::navigation_extra::page_count(
            filtered_table_indices(&self.columns,&self.rows,&self.active_filters).len(),
            self.page_size)
    }
}
impl RenderOnce for AntTable{
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement{
        let t=self.theme;
        let overrides=t.component(AntComponent::Table);
        let fg=overrides.color_text.unwrap_or(t.text);
        let surface=overrides.color_bg.unwrap_or(t.surface);
        let header_bg=overrides.color_header_bg.unwrap_or(t.surface_secondary);
        let border=overrides.color_border.unwrap_or(t.border);
        let chosen_bg=overrides.color_bg_selected.unwrap_or(t.info_bg);
        let radius=overrides.border_radius.unwrap_or(t.border_radius);
        let font_size=overrides.font_size.unwrap_or(t.font_size);
        let prefix=format!("{:?}",self.id);
        let ordered=table_view_indices(&self.columns,&self.rows,&self.active_filters,
            self.sort_key.as_deref(),self.sort);
        let count=ordered.len();
        let pages=crate::navigation_extra::page_count(count,self.page_size);
        let current=self.page.clamp(1,pages);
        let indexes:Vec<usize>=if self.pagination{
            let start=current.saturating_sub(1).saturating_mul(self.page_size);
            ordered.into_iter().skip(start).take(self.page_size).collect()
        }else{ordered};
        let visible_keys:Vec<String>=indexes.iter().filter_map(|index|{
            let row=&self.rows[*index];
            (!row.disabled).then(||row.key.to_string())
        }).collect();
        let page_checked=!visible_keys.is_empty()
            && visible_keys.iter().all(|key|self.selected_keys.contains(key));

        let mut head=KitRow::new().bg(header_bg);
        if self.selectable {
            let listener=self.on_selection.clone();
            let selected=self.selected_keys.clone();
            head=head.child(TableHead::new().w(px(160.))
                .child(AntCheckbox::new(format!("{prefix}:select-page"),"Page")
                    .theme(t).checked(page_checked).disabled(visible_keys.is_empty())
                    .on_change(move|next,window,cx|{
                        if let Some(listener)=&listener{
                            listener(toggle_table_visible_selection(
                                &visible_keys,&selected,next),window,cx);
                        }
                    })));
        }
        for col in &self.columns {
            let key=col.key.clone();
            let active_sort=self.sort_key.as_ref()==Some(&col.key);
            let direction=if active_sort{self.sort}else{TableSort::None};
            let title=if col.sortable{
                format!("{} {}",col.title,direction.mark())
            }else{col.title.to_string()};
            let mut content=div().flex().items_center().gap(px(6.));
            if col.sortable{
                let listener=self.on_sort.clone();
                let sort_key=key.clone();
                content=content.child(
                    AntButton::new(format!("{prefix}:sort:{key}"),title)
                        .size(ControlSize::Small).theme(t).text()
                        .when_some(listener,|this,cb|{
                            this.on_click(move|_,window,cx|{
                                cb(sort_key.clone(),direction.next(),window,cx);
                            })
                        })
                );
            }else{
                content=content.child(
                    div().text_color(fg).font_medium().child(title)
                );
            }
            if !col.filters.is_empty() {
                let options=col.filters.clone();
                let current_filter=self.active_filters.get(key.as_ref()).cloned();
                let display=current_filter.clone().unwrap_or_else(||"All".to_owned());
                let mut dropdown=AntDropdown::new(format!("{prefix}:filter:{key}"),
                    format!("Filter: {display}")).theme(t).min_width(132.)
                    .item(AntDropdownItem::new("all","All")
                        .checked(current_filter.is_none()));
                for (index,value) in options.iter().enumerate(){
                    dropdown=dropdown.item(
                        AntDropdownItem::new(format!("value:{index}"),value.clone())
                            .checked(current_filter.as_deref()==Some(value.as_ref()))
                    );
                }
                let callback=self.on_filter_change.clone();
                let filter_key=key.clone();
                dropdown=dropdown.on_select(move|id,window,cx|{
                    if let Some(callback)=&callback{
                        let requested=id.as_ref().strip_prefix("value:")
                            .and_then(|text|text.parse::<usize>().ok())
                            .and_then(|index|options.get(index).cloned());
                        callback(filter_key.clone(),requested,window,cx);
                    }
                });
                content=content.child(dropdown);
            }
            head=head.child(TableHead::new().w(px(col.width.unwrap_or(160.)))
                .bg(header_bg).child(content));
        }

        let mut body=TableBody::new();
        if indexes.is_empty(){
            body=body.child(KitRow::new().child(TableCell::new()
                .child(div().text_size(px(14.)).text_color(t.text_secondary)
                    .child("No matching records"))));
        }
        for (position,ix) in indexes.into_iter().enumerate(){
            let record=&self.rows[ix];
            let checked=self.selected_keys.contains(record.key.as_ref());
            let mut row=KitRow::new()
                .when(self.striped&&position%2==1,|this|this.bg(t.surface_secondary))
                .when(checked,|this|this.bg(chosen_bg));
            if self.selectable {
                let key=record.key.to_string();
                let selected=self.selected_keys.clone();
                let listener=self.on_selection.clone();
                row=row.child(TableCell::new().w(px(160.))
                    .child(AntCheckbox::new(format!("{prefix}:select:{key}"),"Select")
                        .theme(t).checked(checked).disabled(record.disabled)
                        .on_change(move|next,window,cx|{
                            if let Some(listener)=&listener {
                                let mut keys=selected.clone();
                                if next{keys.insert(key.clone());}else{keys.remove(&key);}
                                listener(keys,window,cx);
                            }
                        })));
            }
            for (cell_ix,col) in self.columns.iter().enumerate(){
                let value=record.cells.get(cell_ix).cloned().unwrap_or_default();
                row=row.child(TableCell::new().w(px(col.width.unwrap_or(160.)))
                    .child(div().text_color(if record.disabled{t.disabled_text}else{fg})
                        .text_size(px(font_size)).child(value)));
            }
            body=body.child(row);
        }
        let table=KitTable::new()
            .accessibility_label("Ant Design data table")
            .w_full().bg(surface).text_color(fg)
            .when(self.bordered,|this|this.border_1().border_color(border))
            .rounded(px(radius))
            .child(TableHeader::new().bg(header_bg).child(head))
            .child(body);
        let page_change=self.on_page_change;
        div().w_full().flex().flex_col().gap(px(14.))
            .child(table)
            .when(self.pagination,|this|{
                this.child(AntPagination::new(format!("{prefix}:pager"),count)
                    .page_size(self.page_size).current(current).show_total(true)
                    .size(ControlSize::Small).theme(t)
                    .when_some(page_change,|this,cb|this.on_change(move|page,_,window,cx|{
                        cb(page,window,cx)
                    })))
            })
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    fn fixture()->(Vec<AntTableColumn>,Vec<AntTableRow>){
        (vec![AntTableColumn::new("name","Name").sortable(true),
            AntTableColumn::new("age","Age").sortable(true).numeric(true)],
         vec![AntTableRow::new("a").cells(["Ada","9"]),
              AntTableRow::new("b").cells(["Bob","101"]),
              AntTableRow::new("c").cells(["Clara","21"])])
    }
    #[test]
    fn column_sorts_are_stable_and_numeric(){
        let (cols,rows)=fixture();
        assert_eq!(sorted_table_indices(&cols,&rows,Some("age"),TableSort::Ascending),vec![0,2,1]);
        assert_eq!(sorted_table_indices(&cols,&rows,Some("age"),TableSort::Descending),vec![1,2,0]);
        assert_eq!(sorted_table_indices(&cols,&rows,None,TableSort::Ascending),vec![0,1,2]);
        assert_eq!(TableSort::None.next(),TableSort::Ascending);
        assert_eq!(TableSort::Ascending.next(),TableSort::Descending);
        assert_eq!(TableSort::Descending.next(),TableSort::None);
    }
    #[test]
    fn table_page_math_is_bounded(){
        let table=AntTable::new("table").page_size(2)
            .row(AntTableRow::new("a"))
            .row(AntTableRow::new("b"))
            .row(AntTableRow::new("c"));
        assert_eq!(table.page_count(),2);
        assert_eq!(AntTable::new("empty").page_count(),1);
    }

    #[test]
    fn table_filters_before_sorting_and_paging(){
        let cols=vec![AntTableColumn::new("name","Name").sortable(true),
            AntTableColumn::new("status","Status").filters(["Active","Pending"]),
            AntTableColumn::new("score","Score").sortable(true).numeric(true)];
        let rows=vec![
            AntTableRow::new("a").cells(["Ada","Active","30"]),
            AntTableRow::new("b").cells(["Bob","Pending","90"]),
            AntTableRow::new("c").cells(["Clara","Active","10"]),
            AntTableRow::new("d").cells(["Drew","Active","70"])];
        let filters=BTreeMap::from([("status".to_owned(),"Active".to_owned())]);
        assert_eq!(filtered_table_indices(&cols,&rows,&filters),vec![0,2,3]);
        assert_eq!(table_view_indices(&cols,&rows,&filters,
            Some("score"),TableSort::Descending),vec![3,0,2]);
        let table=cols.into_iter().fold(AntTable::new("filtered").page_size(2),
            |table,col|table.column(col))
            .rows(rows).active_filters([("status","Active")]);
        assert_eq!(table.page_count(),2);
    }
    #[test]
    fn unrecognized_table_filters_do_not_hide_every_row(){
        let(cols,rows)=fixture();
        let unknown=BTreeMap::from([("missing".to_owned(),"value".to_owned())]);
        assert_eq!(filtered_table_indices(&cols,&rows,&unknown),vec![0,1,2]);
    }
    #[test]
    fn select_all_changes_only_visible_enabled_keys(){
        let selected=BTreeSet::from(["hidden".to_owned()]);
        let keys=vec!["a".to_owned(),"c".to_owned()];
        let chosen=toggle_table_visible_selection(&keys,&selected,true);
        assert_eq!(chosen,BTreeSet::from(["a".into(),"c".into(),"hidden".into()]));
        assert_eq!(toggle_table_visible_selection(&keys,&chosen,false),selected);
    }

}
