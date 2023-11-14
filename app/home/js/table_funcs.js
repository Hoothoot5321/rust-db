function add_table_column() {
  let header_row =
    great_table.firstElementChild.firstElementChild.firstElementChild;

  let last_child = header_row.lastElementChild;

  header_row.removeChild(last_child);

  let new_div = document.createElement("div");

  let new_input = document.createElement("input");

  new_input.classList.add("table_column");

  new_div.appendChild(new_input);

  let del_button = document.createElement("button");

  del_button.innerHTML = "Delete Column";

  del_button.style.backgroundColor = "red";

  del_button.addEventListener("click", () => {
    header_row.removeChild(del_button.parentElement);
  });

  new_div.appendChild(del_button);

  header_row.appendChild(new_div);

  let new_button = document.createElement("button");

  new_button.id = "add_table";

  new_button.addEventListener("click", add_table_column);

  new_button.innerHTML = "Add Column";

  header_row.appendChild(new_button);
}
