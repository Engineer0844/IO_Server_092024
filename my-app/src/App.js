import { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';


import logo from './logo.svg';
import './App.css';

const WS_URL = 'ws://' + window.location.host + '/ws';



function LabelDisplay() {
  
  const [value, setValue] = useState(10);
  const { sendMessage, lastMessage, readyState } = useWebSocket(WS_URL, {
    onOpen: () => {
      console.log('WebSocket connection established.');
    },
    share: true
  });

  useEffect(() => {
    if (lastMessage !== null) {
      const json_payload = JSON.parse(lastMessage.data);
      if(json_payload["id"] == "adc-1") {
        setValue(json_payload["text"]);
      }
    }
  }, [lastMessage]);

  return (
    <div className="label">
      <p>Label: </p> <p>{value}</p>
    </div>
  );
}

function App() {

    useWebSocket(WS_URL, {
	    onOpen: () => {
	      console.log('WebSocket connection established.');
	    },
      share: true
    });

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
      <div>
        <LabelDisplay />
      </div>
    </div>
  );
}

export default App;
