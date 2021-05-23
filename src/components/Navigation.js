import React from 'react';

class Navigation extends React.Component {
  render() {
    return (
      <nav className="navbar navbar-dark bg-dark" style={{width:'100vw'}}>
        <div className="container-fluid">
          <a className="navbar-brand" href="/" style={{color: 'powderblue', fontSize: '30px'}}>MangaKEKW</a>
          <form className="d-flex">
            <input className="form-control me-2" type="search" placeholder="Search" aria-label="Search" style={{width: '25vw'}} />
            <button className="btn btn-outline-info" type="submit">Search</button>
          </form>
        </div>
      </nav>
    )

  }
}

export default Navigation;
