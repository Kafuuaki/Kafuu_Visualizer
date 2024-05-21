let walker;


function setup() {
  createCanvas(400, 400);
  background(200);
  stroke(200);
  walker = new Walker(width / 2, height / 2);
}

function draw() {
  walker.step();
  walker.display();
}