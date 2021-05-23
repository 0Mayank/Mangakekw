import './App.css';
import Navigation from './components/Navigation.js'
import Cards from './components/Cards.js'
import "bootstrap/dist/css/bootstrap.min.css"

function App() {
  return (
    <div className="App">
      <Navigation />
      <Cards />
      <div class='container-fluid' style={{background: 'powderblue', width: '100vw', height: '100vh'}}></div>
    </div>
  );
}

export default App;
