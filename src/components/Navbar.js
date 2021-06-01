import React from 'react';
import { Navbar } from 'react-bootstrap';
import { Form } from 'react-bootstrap';
import { FormControl } from 'react-bootstrap';
import { Button } from 'react-bootstrap';
import { Row, Col } from 'react-bootstrap';
import Background from './Background';
import { useState } from 'react';
import { Card } from 'react-bootstrap';
import styled, { css } from 'styled-components';
import JSONDATA from '../json/manga.json'
import Schedule from './Schedule';

const Icon = styled.div`
    transition: transform 100ms;
  &:hover {
    transform: scale(1.02);
  }
`;

function Navigation() {
    const [searchTerm, setSearchTerm] = useState('')
    return (
        <>
            <Navbar className="bg-dark justify-content-between">
                <Navbar.Brand href='/' style={{ color: 'white' }}>MangaKEKW</Navbar.Brand>
                <Form inline>
                    <FormControl type="text" placeholder="Search" className="mr-sm-2" onChange={(event) => { setSearchTerm(event.target.value); }} />
                    <Button variant="outline-info">Search</Button>
                </Form>
            </Navbar>
            <Background>
                <Row>
                    <Col xs={7}>
                        <Row>
                            {JSONDATA.filter((val) => {
                                if (searchTerm == "") {
                                    return val
                                } else if (val.cover.toLowerCase().includes(searchTerm.toLowerCase())) {
                                    return val
                                }
                            }).map((val, key) => {
                                return (
                                    <Col lg={4}>
                                        <Card border="dark" className="m-2 p-1" style={{ background: '#2e2e2e' }}>
                                            <Icon>
                                                <Card.Img variant="top" src={val.cover} />
                                                <Card.Footer style={{ color: 'white' }}>Title</Card.Footer>
                                            </Icon>
                                        </Card>
                                    </Col>
                                )
                            })}
                        </Row>
                    </Col>
                    <Col><Schedule /></Col>
                </Row>
            </Background>
        </>
    )
}

export default Navigation;
