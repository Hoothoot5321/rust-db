async function post_table() {
  let db_title = input.value;

  let table_name = table_input.value;

  let new_url = url + "api/" + db_title;

  let temp_headers = great_table.querySelectorAll(".table_column");

  let headers = [];

  for (let i = 0; i < temp_headers.length; i++) {
    headers.push(temp_headers[i].value);
  }

  let res = await fetch(new_url, {
    method: "POST",
    body: JSON.stringify({
      name: table_name,
      headers: headers,
    }),
  });

  let body = await handle_resp(res);

  if (!body) {
    return;
  }
}

async function get_table() {
  let db_title = input.value;

  let table_name = table_input.value;

  let new_url = url + "api/" + db_title + "/" + table_name;

  let res = await fetch(new_url);

  let body = await handle_resp(res);

  if (!body) {
    return;
  }

  let headers = Object.keys(body);

  headers = headers.reverse();

  let t_body = confirmed_table.firstElementChild;
  t_body.innerHTML = "";
  t_body.appendChild(document.createElement("tr"));

  let first = t_body.firstElementChild;
  for (let i = 0; i < headers.length; i++) {
    let head = document.createElement("th");

    let head_input = document.createElement("input");

    head_input.value = headers[i];

    head_input.style.fontWeight = "bold";

    head.appendChild(head_input);

    first.appendChild(head);
  }
  for (let i = 0; i < body[headers[0]].length; i++) {
    let row = document.createElement("tr");
    for (let b = 0; b < headers.length; b++) {
      let elem = document.createElement("td");

      let elem_input = document.createElement("input");

      elem_input.classList.add(headers[b]);

      elem_input.value = body[headers[b]][i];

      elem.appendChild(elem_input);

      row.appendChild(elem);
    }
    t_body.appendChild(row);

    table_selector.value = "update";
    table_selector.dispatchEvent(new Event("change"));
  }
}

function add_row() {
  let header_elements = confirmed_table.querySelectorAll("th");

  let header = [];

  for (let i = 0; i < header_elements.length; i++) {
    let val = header_elements[i].firstElementChild.value;

    header.push(val);
  }

  let t_body = confirmed_table.firstElementChild;

  let row = document.createElement("tr");
  for (let i = 0; i < header.length; i++) {
    let elem = document.createElement("td");

    let elem_input = document.createElement("input");

    elem_input.classList.add(header[i]);

    elem.appendChild(elem_input);

    row.appendChild(elem);
  }
  t_body.appendChild(row);
}

async function update_table() {
  let db_title = input.value;

  let table_title = table_input.value;

  let new_url = url + "api/" + db_title + "/" + table_title;

  let header_elements = confirmed_table.querySelectorAll("th");

  let headers = [];

  for (let i = 0; i < header_elements.length; i++) {
    let val = header_elements[i].firstElementChild.value;

    headers.push(val);
  }

  let body = {};

  for (let i = 0; i < headers.length; i++) {
    let elem_inputs = confirmed_table.querySelectorAll("." + headers[i]);

    let elem = [];

    for (let b = 0; b < elem_inputs.length; b++) {
      elem.push(elem_inputs[b].value);
    }

    body[headers[i]] = elem;
  }

  let res = await fetch(new_url, {
    method: "PUT",
    body: JSON.stringify(body),
  });
}
