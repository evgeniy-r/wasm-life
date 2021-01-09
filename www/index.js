import { Board } from 'life';
import { getStartPosition } from "./start-position";

const board = Board.for_canvas('canvas', 1600, 800);
board.load(getStartPosition(), 100, 400);
// If you want random filling instead
// board.fill_with_random(400, 0.5);
board.render();

const turnLabel = document.getElementById('turn');

const next = count => {
  // console.time(`next${count}`);
  for (let i = 0; i < count; i++) {
    board.next();
  }
  // console.timeEnd(`next${count}`);

  // console.time('render');
  board.render();
  // console.timeEnd('render');

  turnLabel.textContent = board.turn.toString();
}

let animationId = null;

const renderLoop = () => {
  next(1);
  animationId = requestAnimationFrame(renderLoop);
};

const playPauseBtn = document.getElementById("playPauseBtn");

playPauseBtn.addEventListener('click', () => {
  if (animationId === null) {
    playPauseBtn.textContent = '⏸';
    renderLoop();
  }
  else {
    playPauseBtn.textContent = '▶';
    cancelAnimationFrame(animationId);
    animationId = null;
  }
});

document.getElementById('nextBtn1').addEventListener('click', () => next(1));
document.getElementById('nextBtn10').addEventListener('click', () => next(10));
document.getElementById('nextBtn100').addEventListener('click', () => next(100));
