class Walker {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
  
    display() {
        stroke(0);
        point(this.x, this.y);
    }
  
    step() {
        let stepx = random(-2, 2);
        let stepy = random(-2, 2);
        this.x += stepx;
        this.y += stepy;
    }
  }