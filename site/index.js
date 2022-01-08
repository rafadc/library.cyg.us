function addToTable(element) {
    console.log("Row:", element);
    const newTRow = document.createElement("tr");

    const title = document.createElement("td");
    title.innerText = element[0];
    newTRow.appendChild(title);

    const lastUpdatedAt = document.createElement("td");
    lastUpdatedAt.innerText = element[1];
    newTRow.appendChild(lastUpdatedAt);

    const finishedReadingAt = document.createElement("td");
    finishedReadingAt.innerText = element[2];
    newTRow.appendChild(finishedReadingAt);

    document.getElementById("book_table_body").appendChild(newTRow);
}

document.addEventListener("DOMContentLoaded", function() {

    const initSqlJs = window.initSqlJs;

    initSqlJs({
        // Required to load the wasm binary asynchronously. Of course, you can host it wherever you want
        // You can omit locateFile completely when running in node
        locateFile: file => 'https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.6.1/sql-wasm.wasm'
    }).then(SQL => {
        xhr = new XMLHttpRequest();

        xhr.open("GET", "/metadata/books.sqlite", true);
        xhr.responseType = "arraybuffer";

        xhr.onload = function(e) {
            let database_data = new Uint8Array(xhr.response);
            const db = new SQL.Database(database_data);

            let sqlstr = "SELECT title, finished_at, last_updated_at FROM books;";

            const result = db.exec(sqlstr, {});

            result[0].values.forEach(addToTable);

            $('#book_list').DataTable( {
                "info":     false
            } );
        };

        xhr.send();
    });
});
