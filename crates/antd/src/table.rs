//! Small to medium Ant Design-style Table using the native GPUI Kit Table
//! semantics. Column sort, paged visible rows and row selection are controlled.
//!
//! This is NOT the high-volume virtualized DataTable: there is no fixed
//! header/column, resizable columns, filter popup, tree row or async source.
//! Use GPUI Kit DataTable or the independent gpui-grid for large datasets.
use std::{cmp::Ordering,collections::BTreeSet,rc::Rc};
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
use crate::{AntButton,AntCheckbox,AntPagination,AntTheme,ControlSize};

#[derive(Debug,Clone)]
pub struct AntTableColumn{
    key:SharedString,
    title:SharedString,
    width:Option<f32>,
    sortable:bool,
    numeric:bool,
}
impl AntTableColumn{
    pub fn new(key:impl Into<SharedString>,title:impl Into<SharedString>)->Self{
        Self{key:key.into(),title:title.into(),width:None,sortable:false,numeric:false}
    }
    pub fn width(mut self,width:f32)->Self{
        self.width=Some(if width.is_finite(){width.max(100.)}else{160.});self
    }
    pub fn sortable(mut self,yes:bool)->Self{self.sortable=yes;self}
    pub fn numeric(mut self,yes:bool)->Self{self.numeric=yes;self}
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
    let mut out:(Vec<usize>)=(0..rows.len()).collect();
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

type SortHandler=Rc<dyn Fn(SharedString,TableSort,&mut Window,&mut App)>;
type SelectionHandler=Rc<dyn Fn(BTreeSet<String>,&mut Window,&mut App)>;
type PageHandler=Rc<dyn Fn(usize,&mut Window,&mut App)>;

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
}
impl AntTable {
    pub fn new(id:impl Into<ElementId>)->Self{
        Self{id:id.into(),columns:Vec::new(),rows:Vec::new(),
            sort_key:None,sort:TableSort::None,selected_keys:BTreeSet::new(),
            selectable:false,striped:false,bordered:true,pagination:true,
            page:1,page_size:10,theme:AntTheme::default(),
            on_sort:None,on_selection:None,on_page_change:None}
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
    pub fn page_count(&self)->usize{crate::navigation_extra::page_count(self.rows.len(),self.page_size)}
}
impl RenderOnce for AntTable{
    fn render(self,_:&mut Window,_:&mut App)->impl IntoElement{
        let t=self.theme;
        let prefix=format!("{:?}",self.id);
        let current=self.page.clamp(1,self.page_count());
        let count=self.rows.len();
        let ordered=sorted_table_indices(&self.columns,&self.rows,
            self.sort_key.as_deref(),self.sort);
        let mut head=KitRow::new();
        if self.selectable {
            head=head.child(TableHead::new().w(px(150.))
                .child("Select").text_color(t.text_secondary));
        }
        for col in &self.columns {
            let key=col.key.clone();
            let active=self.sort_key.as_ref()==Some(&col.key);
            let status=if active{self.sort}else{TableSort::None};
            let title=if col.sortable {
                format!("{} {}",col.title,status.mark())
            }else{col.title.to_string()};
            let content=if col.sortable{
                let listener=self.on_sort.clone();
                AntButton::new(format!("{prefix}:sort:{key}"),title)
                    .size(ControlSize::Small).theme(t).text()
                    .when_some(listener,|this,cb|{
                        this.on_click(move|_,window,cx|{
                            cb(key.clone(),status.next(),window,cx);
                        })
                    }).into_any_element()
            }else{
                div().text_color(t.text).font_medium().child(title).into_any_element()
            };
            head=head.child(TableHead::new().w(px(col.width.unwrap_or(160.)))
                .bg(t.surface_secondary).child(content));
        }
        let mut body=TableBody::new();
        let indexes:Vec<usize>=if self.pagination{
            let start=current.saturating_sub(1).saturating_mul(self.page_size);
            ordered.into_iter().skip(start).take(self.page_size).collect()
        }else{ordered};
        if indexes.is_empty(){
            body=body.child(KitRow::new().child(TableCell::new()
                .child(div().text_size(px(14.)).text_color(t.text_secondary)
                    .child("No data"))));
        }
        for (position,ix) in indexes.into_iter().enumerate(){
            let record=&self.rows[ix];
            let checked=self.selected_keys.contains(record.key.as_ref());
            let mut row=KitRow::new()
                .when(self.striped&&position%2==1,|this|this.bg(t.surface_secondary))
                .when(checked,|this|this.bg(t.info_bg));
            if self.selectable {
                let key=record.key.to_string();
                let selected=self.selected_keys.clone();
                let listener=self.on_selection.clone();
                row=row.child(TableCell::new().w(px(150.))
                    .child(AntCheckbox::new(format!("{prefix}:select:{key}"),"Select")
                        .theme(t).checked(checked).disabled(record.disabled)
                        .on_change(move|next,window,cx|{
                            if let Some(listener)=&listener {
                                let mut keys=selected.clone();
                                if next {keys.insert(key.clone());}else{keys.remove(&key);}
                                listener(keys,window,cx);
                            }
                        })));
            }
            for (cell_ix,col) in self.columns.iter().enumerate(){
                let value=record.cells.get(cell_ix).cloned().unwrap_or_default();
                row=row.child(TableCell::new().w(px(col.width.unwrap_or(160.)))
                    .child(div().text_color(if record.disabled{t.disabled_text}else{t.text})
                        .text_size(px(14.)).child(value)));
            }
            body=body.child(row);
        }
        let table=KitTable::new()
            .accessibility_label("Ant Design data table")
            .w_full().bg(t.surface).text_color(t.text)
            .when(self.bordered,|this|this.border_1().border_color(t.border))
            .rounded(px(t.border_radius))
            .child(TableHeader::new().bg(t.surface_secondary).child(head))
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
}
