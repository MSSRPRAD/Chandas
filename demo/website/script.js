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

  let letterBoxes = document.querySelectorAll(".letter-box");
  let currentBox = 0;
  let letters = "";
  let special = ['Shift', 'Enter', 'Space'];
  document.addEventListener("keydown", function(event){
    if(special.includes(event.key)){
      return;
    }
    console.log(event);
    const key = event.key;
    const isVowel = "aAiIuUeaoauRRlRlRR".includes(key);
    if(isVowel) {
      letters += key;
      letterBoxes[currentBox].textContent = letters;
      currentBox++;
      letters = ""
    } else {
      letterBoxes[currentBox].textContent = letters;
      letters += key;
    }
  });

  letterBoxes.forEach(
    (box,index) => {
      box.addEventListener("click", function(event){
        currentBox = index;
        letters = box.textContent;
      });
    }
  )
}
initBoard()
