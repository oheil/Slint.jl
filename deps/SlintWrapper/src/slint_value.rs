//use slint::StandardListViewItem;
use log::*;

//
// SlintValue is the central value type for all Slint models
//      see CellsModel and RowModel below
//      JRvalue is the corresponding type to Julia
//
#[derive(Clone)]
pub struct SlintValue {
    pub value_s: String,
    pub value_i: i32,
    pub value_f: f64,
    //pub value_slvi: StandardListViewItem,
}
impl Default for SlintValue {
    fn default() -> SlintValue {
        debug!("SlintValue default");
        SlintValue{
            value_s: String::from(""),
            value_i: 0,
            value_f: 0.0,
            //value_slvi: StandardListViewItem::from(""),
        }
    }
}
/*
impl From<SlintValue> for StandardListViewItem {
    fn from(item: SlintValue) -> StandardListViewItem {
        item.value_slvi.clone()
    }
}
pub fn convert_vecs<SlintValue, StandardListViewItem>(vector: Vec<SlintValue>) -> Vec<StandardListViewItem>
  where
    SlintValue: TryInto<StandardListViewItem>,
    <SlintValue as std::convert::TryInto<StandardListViewItem>>::Error: std::fmt::Display
{
	vector
        .into_iter()
        .map(|value_t| {
            match TryInto::try_into(value_t) {
                Ok(value_u) => value_u,
                Err(why) => {
                    let t = std::any::type_name::<SlintValue>();
					let u = std::any::type_name::<StandardListViewItem>();
					panic!("Error converting from {t} to {u}: {why}")
				}
			}
        }
    ).collect()
}
*/