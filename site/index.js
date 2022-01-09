function addToTable(element) {
    console.log("Row:", element);
    const newTRow = document.createElement("tr");

    const title = document.createElement("td");
    const link = document.createElement("a");

    link.innerText = element[0];
    link.href = `/books/${element[1]}.html`;
    title.appendChild(link);
    newTRow.appendChild(title);

    const finishedReadingAt = document.createElement("td");
    finishedReadingAt.innerText = element[2];
    newTRow.appendChild(finishedReadingAt);

    const lastUpdatedAt = document.createElement("td");
    lastUpdatedAt.innerText = element[3];
    newTRow.appendChild(lastUpdatedAt);

    document.getElementById("book_table_body").appendChild(newTRow);
}

document.addEventListener("DOMContentLoaded", function() {

    const initSqlJs = window.initSqlJs;

    initSqlJs({
        locateFile: file => 'https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.6.1/sql-wasm.wasm'
    }).then(SQL => {
        xhr = new XMLHttpRequest();

        xhr.open("GET", "/metadata/books.sqlite", true);
        xhr.responseType = "arraybuffer";

        xhr.onload = function(e) {
            let database_data = new Uint8Array(xhr.response);
            const db = new SQL.Database(database_data);

            let sqlstr = "SELECT title, slug, finished_at, last_updated_at FROM books;";

            const result = db.exec(sqlstr, {});

            result[0].values.forEach(addToTable);

            $('#book_list').DataTable( {
                "info": false,
                "order": [[2, 'desc']]
            } );
        };

        xhr.send();
    });
});
