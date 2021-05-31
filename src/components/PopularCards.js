import React from 'react';
import { CardDeck, Card } from 'react-bootstrap';
import { Row, Col } from 'react-bootstrap';

class PopularCards extends React.Component {
    render() {
        return (
            <>
                <Row>
                    <CardDeck>
                        <Col md={3}>
                            <Card border="info" className="m-3">
                                <Card.Img variant="top" src="cover.png" />
                            </Card>
                        </Col>
                    </CardDeck>
                </Row>

            </>
        )
    }
}

export default PopularCards;
