import React from 'react-bootstrap';
import { Container } from 'react-bootstrap';

export const Background = (props) => (
    <Container fluid style={{background: '#555555'}}>
        {props.children}
    </Container>
)