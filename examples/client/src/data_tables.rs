use yew::{html, Html};
use yew_mdc_widgets::{DataTable, TableCell};

pub fn view() -> Html {
    html! {
        <div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Data Table Standard" }</h3>
                <div>{
                    DataTable::new("data-table-standard")
                        .head(vec![
                            TableCell::text("Dessert"),
                            TableCell::num("Carbs (g)"),
                            TableCell::num("Protein (g)"),
                            TableCell::text("Comments"),
                        ])
                        .row(vec![
                            TableCell::text("Frozen yogurt"),
                            TableCell::num("24"),
                            TableCell::num("4.0"),
                            TableCell::text("Super tasty"),
                        ])
                        .row(vec![
                            TableCell::text("Ice cream sandwich"),
                            TableCell::num("37"),
                            TableCell::num("4.33333333333"),
                            TableCell::text("I like ice cream more"),
                        ])
                        .row(vec![
                            TableCell::text("Eclair"),
                            TableCell::num("24"),
                            TableCell::num("6.0"),
                            TableCell::text("New filing flavor"),
                        ])
                }</div>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Data Table with Row Selection" }</h3>
                <div>{
                    DataTable::new("data-table-selection")
                        .row_selection(true)
                        .head(vec![
                            TableCell::text("Dessert"),
                            TableCell::num("Carbs (g)"),
                            TableCell::num("Protein (g)"),
                            TableCell::text("Comments"),
                        ])
                        .row(vec![
                            TableCell::text("Frozen yogurt"),
                            TableCell::num("24"),
                            TableCell::num("4.0"),
                            TableCell::text("Super tasty"),
                        ])
                        .row(vec![
                            TableCell::text("Ice cream sandwich"),
                            TableCell::num("37"),
                            TableCell::num("4.33333333333"),
                            TableCell::text("I like ice cream more"),
                        ])
                        .row(vec![
                            TableCell::text("Eclair"),
                            TableCell::num("24"),
                            TableCell::num("6.0"),
                            TableCell::text("New filing flavor"),
                        ])
                }</div>
            </div>
        </div>
    }
}
