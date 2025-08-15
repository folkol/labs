<canvas id=canvas width=500 height=500 style="border:1px solid black"></canvas>


const TABLE = 2;
const INCREMENT = 1.5;
const SPEED = 1000/60;
const R = 400;
const LINE_COLOR = 'cyan';
const LINE_WIDTH = 0.1;
const NUM_STROKES = 1;
const MOD = 666;

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext("2d");
const X = canvas.width/2;
const Y = canvas.height/2;

ctx.fillStyle = "black";
ctx.fillRect(0, 0, canvas.width, canvas.height);

function coord(n) {
	const v = 2*Math.PI/MOD*n;
	return [X + Math.cos(v) * R, Y - Math.sin(v) * R];
}

function circle(x, y, r) {
	ctx.lineWidth = 1;
	ctx.strokeStyle = 'white';
	ctx.beginPath();
	ctx.arc(x, y, r, 0, 2 * Math.PI);
	ctx.stroke();
}

function line(x1, y1, x2, y2) {
	ctx.lineWidth = LINE_WIDTH;
	ctx.strokeStyle = LINE_COLOR;
	ctx.moveTo(x1, y1);
	ctx.lineTo(x2, y2);
	ctx.stroke();
}

function lineFrom(a, b) {
	const [x1, y1] = coord(a);
 	const [x2, y2] = coord(b);

	line(x1, y1, x2, y2);
	circle(X, Y, R);
}

let i = 0;
const interval = setInterval(() => {
	for (let j = 0; j < NUM_STROKES; j++) { 
		lineFrom(i, TABLE * i); i += INCREMENT;
	}
}, SPEED);

