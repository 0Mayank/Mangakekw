import './App.css';
import "bootstrap/dist/css/bootstrap.min.css"
import Navigation from './components/Navbar.js'
import Carouselcard from './components/Carousel.js'
import PopularCards from './components/PopularCards.js'
import { Layout } from './components/Layout'
import { Background } from './components/Background'
import React from 'react';

class App extends React.Component {

  render() {
    return (
      <div className="App">
        <Navigation />
          <Layout>
            <Background>
              <PopularCards />
            </Background>
          </Layout>
      </div>
    );
  }
}

export default App;
