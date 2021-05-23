import './App.css';
import Navigation from './components/Navigation.js'
import BigCard from './components/BigCard.js'
import SmallCards from './components/SmallCards.js'
import "bootstrap/dist/css/bootstrap.min.css"
import React from 'react';

class App extends React.Component {

  // get_cards(n)
  // {
  //   let cards = []
  //   for(let i=0; i<n; i++)
  //     cards.push(<Cards/>)
  //   return cards
  // }

  render() {
    return (
      <div className="App">
        <Navigation />
        <BigCard />
        <SmallCards />
        <div className='container-fluid' style={{background:'#37393e', width: '100vw', height: '100vh' }}></div>
      </div>
    );
  }
}

export default App;
