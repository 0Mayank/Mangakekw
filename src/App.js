import './App.css';
import "bootstrap/dist/css/bootstrap.min.css"
import Navigation from './components/Navbar.js'
import React from 'react';
import Background from './components/Background';

class App extends React.Component {

  render() {
    return (
      <div className="App">
        {/* <Background> */}
          <Navigation />
        {/* </Background> */}
      </div>
    );
  }
}

export default App;
