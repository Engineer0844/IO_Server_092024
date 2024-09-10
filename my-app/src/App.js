import { useState } from 'react';
import useWebSocket from 'react-use-websocket';


import logo from './logo.svg';
import './App.css';

function Square({ value, onSquareClick }) {
    return (
	    <button className="square" onClick={onSquareClick}>
	    {value}
	</button>
    );
}

function Board({ xIsNext, squares, onPlay }) {
    function handleClick(i) {
	if(calculateWinner(squares) || squares[i]) {
	    
	}
    }

    let status = 'winner';


    return (
	    <>
	    <div className="status">{status}</div>
	    <div className="board-row">
	      <Square value={squares[0]} onSquareClick={() => handleClick(0)} />
	      <Square value={squares[1]} onSquareClick={() => handleClick(1)} />
	      <Square value={squares[2]} onSquareClick={() => handleClick(2)} />
	    </div>
	    </>
    );
}

function calculateWinner(squares)  {
    const lines = [
	[0, 1, 2],
	[3, 4, 5],
	[6, 7, 8],
	[0, 3, 6],
	[1, 4, 7],
	[2, 5, 8],
	[0, 4, 8],
	[2, 4, 6]	
    ];
    for (let i = 0; i < lines.length; i++) {
	const [a, b, c] = lines[i];
	if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
	    return squares[a];
	}
    }
    return null;
}

const WS_URL = 'ws://' + window.location.host + '/ws';


function App() {

    useWebSocket(WS_URL, {
	onOpen: () => {
	    console.log('WebSocket connection established.');
	}
    });


    const [history, setHistory] = useState([Array(9).fill(null)]);
    const [currentMove, setCurrentMove] = useState(0);
    const xIsNext = currentMove % 2 === 0;
    const currentSquares = history[currentMove];


    function handlePlay(nextSquares) {
	const nextHistory = [...history.slice(0, currentMove + 1), nextSquares];
    }
    
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
      <div className="game">
        <Board xIsNext={xIsNext} squares={currentSquares} onPlay={handlePlay} />
      </div>
    </div>
  );
}

export default App;
