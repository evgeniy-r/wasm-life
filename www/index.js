import { Board } from 'life';

const board = Board.for_canvas('canvas', 1600, 800);
board.fill_with_random(400, 0.3);
board.render();

const next = count => {
  console.time(`next${count}`);
  for (let i = 0; i < count; i++) {
    board.next();
  }
  console.timeEnd(`next${count}`);

  // console.time('render');
  board.render();
  // console.timeEnd('render');

  document.getElementById('turn').textContent = board.turn.toString();
}

document.getElementById('nextBtn1').addEventListener('click', () => next(1));
document.getElementById('nextBtn10').addEventListener('click', () => next(10));
document.getElementById('nextBtn100').addEventListener('click', () => next(100));
