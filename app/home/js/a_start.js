let button = document.querySelectorAll("button")[0];

let button2 = document.querySelectorAll("button")[1];

let input = get_elem("#search_string");

let table_input = get_elem("#table_name");

let table = get_elem("#table_list");

let url = "http://localhost:8080/";

let msg_display = get_elem("#msg_display");

let table_button = get_elem("#add_column");

let talbe_creator_button = get_elem("#create_table");

let great_table = get_elem("#table");

let table_selector = get_elem("#table_selector");

let table_state = "create";

let get_table_button = get_elem("#get_table");

let confirmed_table = get_elem("#best_table");

let add_row_button = get_elem("#add_row_button");

let update_table_button = get_elem("#update_table_button");

let creating_div = get_elem("#creating_div");

let updating_div = get_elem("#updating_div");

let database_options = get_elem("#databases");

await get_all_dbs();
