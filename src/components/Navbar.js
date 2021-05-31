import React from 'react';
import { Navbar } from 'react-bootstrap';
import { Form } from 'react-bootstrap';
import { FormControl } from 'react-bootstrap';
import { Button } from 'react-bootstrap';

class Navigation extends React.Component {
    render() {
        return (
            <>
                <Navbar className="bg-dark justify-content-between">
                    <Navbar.Brand href='/' style={{ color: 'white' }}>MangaKEKW</Navbar.Brand>
                    <Form inline>
                        <FormControl type="text" placeholder="Search" className="mr-sm-2">
                        <Button variant="outline-info">Search</Button> 
                        </FormControl>
                    </Form>
                </Navbar>
            </>
        )
    }
}

export default Navigation;
