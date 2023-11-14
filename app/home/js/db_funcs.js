async function get_db() {
  let sendo = input.value;
  let resp = await fetch(url + "api/" + sendo);

  let body = await handle_resp(resp);

  if (!body) {
    return;
  }
  let tables = await JSON.parse(body.msg);

  table.innerHTML = "";

  for (let i = 0; i < tables.length; i++) {
    let row = document.createElement("tr");

    row.innerHTML = tables[i];

    table.appendChild(row);
  }
}

async function post_db() {
  let sendo = input.value;

  let resp = await fetch(url + "api/", {
    method: "POST",
    body: JSON.stringify({
      title: sendo,
    }),
  });
  let body = handle_resp(resp);

  if (!body) {
    return;
  }
  get_all_dbs();
}

async function get_all_dbs() {
  database_options.innerHTML = "";

  let resp = await fetch(url + "api");

  let body = await handle_resp(resp);

  if (!body) {
    return;
  }
  let tables = await JSON.parse(body.msg);

  for (let i = 0; i < tables.length; i++) {
    let row = document.createElement("option");

    row.value = tables[i];

    row.innerHTML = tables[i];

    database_options.appendChild(row);
  }
}
