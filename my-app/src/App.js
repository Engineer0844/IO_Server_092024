import { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';


import logo from './logo.svg';
import './App.css';
import { Button, Table } from 'react-bootstrap';

import 'bootstrap/dist/css/bootstrap.min.css';

const WS_URL = 'ws://' + window.location.host + '/ws';


function TextUpdateRow({ label_name }) {

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
    <tr>
      <td>{label_name}</td>
      <td>{value}</td>
    </tr>
  );
}

function IoStateTable() {
  
  const pin_names = [
    "adc1_channel0",
    "adc1_channel1",
    "adc1_channel2",
    "adc1_channel3",
    "adc2_channel0",
    "adc2_channel1",
    "adc2_channel2",
    "adc2_channel3",
    "pin_one",
    "pin_two"
  ];

  const body = pin_names.map((x) => <TextUpdateRow label_name={x}/>);
  return (<Table striped bordered hover>
      <thead>
        <tr>
          <th>Pin Name</th>
          <th>Value</th>
        </tr>
      </thead>
      <tbody>
        {body}
      </tbody>
    </Table>);
}

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
      <div>
        <p> Marques' cool raspberry pi I/O website</p>
      </div>
      <div>
        <IoStateTable />
      </div>
    </div>
  );
}

export default App;
