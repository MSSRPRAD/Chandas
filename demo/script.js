function initBoard() {
  console.log('reached here');
    let padas = 4
    let pada_length = 10;
    let board = document.getElementById("metre-board");

    for (let i = 0; i < padas; i++) {
        let row = document.createElement("div")
        row.className = "letter-row"

        for (let j = 0; j < pada_length; j++) {
            let box = document.createElement("div")
            box.className = "letter-box"
            row.appendChild(box)
        }

        board.appendChild(row)
    }
}
initBoard()
