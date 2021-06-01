import './App.css';
import "bootstrap/dist/css/bootstrap.min.css"
import Navigation from './components/Navbar.js'
import React from 'react';
import Background from './components/Background';
import Schedule from './components/Schedule';

class App extends React.Component {

  render() {
    return (
      <div className="App">
        {/* <Background> */}
          <Navigation />
          {/* <Schedule /> */}
        {/* </Background> */}
      </div>
    );
  }
}

export default App;
