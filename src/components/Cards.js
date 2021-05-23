import React from 'react'

class Cards extends React.Component {
    render() {
        return (
            <div className='row'>
                <div className='col-8 offset-2'>
                    <div className="card mb-3 mt-3">
                        <img className="card-img-top" src="https://static.bunnycdn.ru/i/cache/images/9/9b/9ba1975bbbbf70dafd9d2fb2603cf5e5.jpg" alt="Card image cap" style={{height: '200px', objectFit: 'cover'}} />
                            <div className="card-body">
                                <h5 className="card-title">Card title</h5>
                                <p className="card-text">Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>
                                <p className="card-text"><small className="text-muted">Last updated 3 mins ago</small></p>
                        </div>
                    </div>
                </div>
            </div>
        )
    }
}

export default Cards;