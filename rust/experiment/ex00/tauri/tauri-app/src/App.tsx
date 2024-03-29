import React from 'react';
import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog'

function App() {
  function openDialog() {
    open().then(files => console.log(files))
  }
  function executeCommands() {
    // invoke('simple_command')
    // invoke('command_with_message', { message: 'some message' }).then(message => {
    //   console.log('command_with_message', message)
    // })
    // invoke('command_with_object', { message: { field_str: 'some message', field_u32: 12 } }).then(message => {
    //   console.log('command_with_object', message)
    // })
    // for (let arg of [1, 2]) {
    //   invoke('command_with_error', { arg }).then(message => {
    //     console.log('command_with_error', message)
    //   }).catch(message => {
    //     console.error('command_with_error', message)
    //   })
    // }
    invoke('async_command', { arg: 14 }).then(message => {
      console.log('async_command', message)
    })
  }
  return (
    <>
      <div id="timer">00:00:000</div>
      <button id="start">Start</button>
      <button id="stop">Strop</button>
      <button id="reset">Reset</button>
      <button id="register" onClick={openDialog}>Register</button>
    </>
  );
  // return (
  //   <div className="App">
  //     <header className="App-header">
  //       <img src={logo} className="App-logo" alt="logo" />
  //       <p>
  //         Edit <code>src/App.tsx</code> and save to reload.
  //         <br></br>
  //         Hello Tauri
  //       </p>
  //       <button onClick={executeCommands}>Click to execute command</button>
  //       <a
  //         className="App-link"
  //         href="https://reactjs.org"
  //         target="_blank"
  //         rel="noopener noreferrer"
  //       >
  //         Learn React
  //       </a>
  //     </header>
  //   </div>
  // );
}

export default App;
