// crate voronoi diagram

let randomPoints = [];

function setup(){
    createCanvas(400, 400);

    for (let i = 0; i < 100; i++){
        randomPoints.push(createVector(random(width), random(height)));
    }
}

function draw(){
    // print a dot in pps5 not log
    background(51);
    let x = random(width);
    let y = random(height);
    // ellipse(x, y, 24, 24);
    triangle(0, 0, 0);

    for (point of randomPoints){
        stroke(0);
        strokeWeight(8);
        point(point.x, point.y);
    }
}



// let xoff = 0;

// function setup() {
//   createCanvas(400, 400);
// }

// function draw() {
//   background(51);
//   // let x = random(width);

//   let x = map(noise(xoff), 0, 1, 0, width);

//   xoff += 0.01;

//   ellipse(x, 200, 24, 24);
// }
