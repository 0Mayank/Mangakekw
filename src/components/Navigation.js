import React from 'react';

class Navigation extends React.Component {
    render() {
        return (
          <nav className="navbar navbar-dark bg-dark" style={{width: "100vw"}}>
            <div className="container-fluid" style={{height: "2px"}}></div>
              <div className="navbar-brand offset-1" style={{fontSize: 30, color: "powderblue"}}>MangaKEKW</div>
                <div className="container offset-5" style={{width: "30vw"}}>
                  <form className="form-inline input-group" style={{background: "rgb(33, 36, 41)"}}>
                    <input className="form-control" type="search" placeholder="Search" aria-label="Search"></input>
                      <button className="btn btn-info input-group-append" type="submit">Search</button>
                  </form>
              </div>
        </nav>
        )

    }
}

export default Navigation;
