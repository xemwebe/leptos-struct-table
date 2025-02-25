#![deny(missing_docs)]
//! Simple showcase example.

use crate::uuid::Uuid;
use async_trait::async_trait;
use chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

#[allow(unused_variables)]
#[component]
pub fn CustomTableRowValueRenderer<K, F>(
    /// The class attribute for the row element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The index of the row. Starts at 0 for the first body row. The header row always has index 0 as well.
    #[prop(into)]
    key: K,
    /// The index of the row. Starts at 0 for the first body row.
    index: usize,
    /// The selected state of the row. True, when the row is selected.
    #[prop(into)]
    selected: Signal<bool>,
    /// The event handler for the click event. Has to be called with [`TableRowEvent`].
    on_click: F,
    /// The raw value for this row. Useful for building columns that require multiple fields
    #[prop(optional)]
    row_value: Signal<Book>,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableRowEvent<K>) + 'static,
    K: Clone + 'static,
{
    view! {
        <tr class=class on:click=move |mouse_event| on_click(TableRowEvent {
            key: key.clone(),
            index,
            mouse_event,
        })>
            {children()}
            <td><a href=move || format!("/edit/{}", row_value().id.to_string())>"Edit"</a></td>
        </tr>
    }
}

/// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(
    //row_value_renderer = "CustomTableRowValueRenderer",
    row_renderer = "CustomTableRowValueRenderer",
    sortable)]
pub struct Book {
    /// Id of the entry.
    #[table(key)]
    pub id: Uuid,
    /// Title of the book.
    pub title: String,
    /// Author of the book.
    pub author: String,
    /// Date when book has been published.
    pub publish_date: Option<NaiveDate>,
    /// Description of the book. Optional.
    #[table(none_value = "-")]
    pub description: Option<String>,
    /// Example on hidden member.
    #[table(skip)]
    pub hidden_field: String,
    /// Example of a headerless column
    #[table(skip_header)]
    pub rating: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let items = create_rw_signal(vec![
            Book {
                id: Uuid::default(),
                title: "The Great Gatsby".to_string(),
                author: "F. Scott Fitzgerald".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1925, 4, 10).unwrap()),
                description: Some(
                    "A story of wealth, love, and the American Dream in the 1920s.".to_string(),
                ),
                hidden_field: "hidden".to_string(),
                rating: "5/5".to_string()
            },
            Book {
                id: Uuid::default(),
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1939, 4, 14).unwrap()),
                description: None,
                hidden_field: "not visible in the table".to_string(),
                rating: "4/5".to_string()
            },
            Book {
                id: Uuid::default(),
                title: "Nineteen Eighty-Four".to_string(),
                author: "George Orwell".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1949, 6, 8).unwrap()),
                description: None,
                hidden_field: "hidden".to_string(),
                rating: "19/84".to_string()
            },
            Book {
                id: Uuid::default(),
                title: "Ulysses".to_string(),
                author: "James Joyce".to_string(),
                publish_date: Some(NaiveDate::from_ymd_opt(1922, 2, 2).unwrap()),
                description: None,
                hidden_field: "hidden".to_string(),
                rating: "really long".to_string()
            },
        ]);

        view! {
            <BookTable items=items />
        }
    })
}
