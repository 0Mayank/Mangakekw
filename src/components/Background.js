import React from 'react-bootstrap';
import { Container } from 'react-bootstrap';
//#161616
export const Background = (props) => (
    <Container fluid style={{background:'papayawhip'}}>
        {props.children}
    </Container>
)
export default Background;