function get_elem(search) {
  return document.querySelector(search);
}

function check_resp_err(resp) {
  if (resp.status !== 200) {
    return false;
  }
  return true;
}

async function handle_resp(resp) {
  msg_display.style.visibility = "visible";
  if (!check_resp_err(resp)) {
    let text = await resp.text();

    msg_display.innerHTML = text;
    return null;
  }
  msg_display.innerHTML = "succes";
  return await resp.json();
}
