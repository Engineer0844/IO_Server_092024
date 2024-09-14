import { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';


import logo from './logo.svg';
import './App.css';

const WS_URL = 'ws://' + window.location.host + '/ws';


// this is text update label
function LabelDisplay({ label_name }) {
  
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
      if(json_payload["id"] == label_name) {
        setValue(json_payload["text"]);
      }
    }
  }, [lastMessage]);

  return (
    <div className="label">
      <p>{label_name}: </p> <p>{value}</p>
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
        <LabelDisplay label_name="adc-1"/>
        <LabelDisplay label_name="adc-2"/>
        <LabelDisplay label_name="adc-3"/>
      </div>
    </div>
  );
}

export default App;
