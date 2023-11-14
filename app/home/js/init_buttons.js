button.addEventListener("click", get_db);

button2.addEventListener("click", post_db);

table_button.addEventListener("click", add_table_column);

talbe_creator_button.addEventListener("click", post_table);

get_table_button.addEventListener("click", get_table);

add_row_button.addEventListener("click", add_row);

update_table_button.addEventListener("click", update_table);

table_selector.addEventListener("change", (e) => {
  let target = e.target;

  switch (target.value) {
    case "update":
      updating_div.style.display = "inline";
      creating_div.style.display = "none";
      break;
    case "create":
      updating_div.style.display = "none";
      creating_div.style.display = "inline";
      break;
  }
});
