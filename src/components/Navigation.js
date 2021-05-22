import { buildQueries } from '@testing-library/dom';
import React from 'react';
import ReactDOM from 'react-dom';

class Navigation extends React.Component {
    render() {
        return (
            <nav class="navbar navbar-dark bg-dark">
              <div class="container-fluid" style={{height: "2px"}}></div>
                <div class="navbar-brand offset-1" style={{fontSize: 30, color: "powderblue"}}>MangaKEKW</div>
                  <div class="container offset-5" style={{width: "30vw"}}>
                    <form class="form-inline input-group" style={{background: "rgb(33, 36, 41)"}}>
                      <input class="form-control" type="search" placeholder="Search" aria-label="Search"></input>
                    <button class="btn btn-info input-group-append" type="submit">Search</button>
                  </form>
                </div>
            </nav>
        )
    }
}

export default Navigation;
