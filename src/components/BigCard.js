import React from 'react'

class BigCard extends React.Component {
    render() {
        return (
            <div className='container-fluid' style={{ background: '#37393e', width: '100vw', height: '100vh' }}>
                <div className='container' style={{background:'#37393e', position:'relative', top:'30px'}}>
                    <div className='row'>
                        <div className='col-8'>
                            <div id="carouselExampleCaptions" className="carousel slide" data-bs-ride="carousel">
                                <div className="carousel-indicators">
                                    <button type="button" data-bs-target="#carouselExampleCaptions" data-bs-slide-to="0" className="active" aria-current="true" aria-label="Slide 1"></button>
                                    <button type="button" data-bs-target="#carouselExampleCaptions" data-bs-slide-to="1" aria-label="Slide 2"></button>
                                    <button type="button" data-bs-target="#carouselExampleCaptions" data-bs-slide-to="2" aria-label="Slide 3"></button>
                                </div>
                                <div className="carousel-inner" style={{ height: '65vh', borderRadius: '10px'}}>
                                    <div className="carousel-item active">
                                        <img src="https://static.bunnycdn.ru/i/cache/images/5/58/5806a16f2892768b4930c39ebf6ce756.jpg" className="d-block w-100" alt="..." />
                                        <div className="carousel-caption d-none d-md-block" style={{ background: '#212429' }}>
                                            <h5>First slide label</h5>
                                            <p>Some representative placeholder content for the first slide.</p>
                                        </div>
                                    </div>
                                    <div className="carousel-item">
                                        <img src="https://static.bunnycdn.ru/i/cache/images/9/9b/9ba1975bbbbf70dafd9d2fb2603cf5e5.jpg" className="d-block w-100" alt="..." />
                                        <div className="carousel-caption d-none d-md-block" style={{ background: '#212429' }}>
                                            <h5>Second slide label</h5>
                                            <p>Some representative placeholder content for the second slide.</p>
                                        </div>
                                    </div>
                                    <div className="carousel-item">
                                        <img src="https://static.bunnycdn.ru/i/cache/images/2019/08/351526352e88e8de1e7b5008d52543e7.jpg" className="d-block w-100" alt="..." />
                                        <div className="carousel-caption d-none d-md-block" style={{ background: '#212429' }}>
                                            <h5>Third slide label</h5>
                                            <p>Some representative placeholder content for the third slide.</p>
                                        </div>
                                    </div>
                                </div>
                                <button className="carousel-control-prev" type="button" data-bs-target="#carouselExampleCaptions" data-bs-slide="prev">
                                    <span className="carousel-control-prev-icon" aria-hidden="true"></span>
                                    <span className="visually-hidden"></span>
                                </button>
                                <button className="carousel-control-next" type="button" data-bs-target="#carouselExampleCaptions" data-bs-slide="next">
                                    <span className="carousel-control-next-icon" aria-hidden="true"></span>
                                    <span className="visually-hidden"></span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        )

    }
}

export default BigCard;