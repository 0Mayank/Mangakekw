import React from 'react';
import { ButtonGroup } from 'react-bootstrap';
import { Button } from 'react-bootstrap';
import { Container } from 'react-bootstrap';
import scheduleData from '../json/schedule.json';


class Schedule extends React.Component {
    state = {
        scheduleDay : "monday"
    }

    constructor(props) {
        super(props);
        this.getSchedule = this.getSchedule.bind(this);
    }

    getSchedule(e) {
        e.preventDefault()
        let newScheduleDay = e.target.name
        this.setState({ scheduleDay : newScheduleDay })
    }
    
    
    render() {
        const scheduleDay = this.state.scheduleDay
        console.log(scheduleData)
        return (
            <>
                <ButtonGroup className="mb-2">
                    <Button variant="dark" name="monday" onClick={(e)=>{this.getSchedule(e)}}>Monday</Button>
                    <Button variant="dark" name="tuesday" onClick={(e)=>{this.getSchedule(e)}}>Tuesday</Button>
                    <Button variant="dark" name="wednesday" onClick={(e)=>{this.getSchedule(e)}}>Wednesday</Button>
                    <Button variant="dark" name="thursday" onClick={(e)=>{this.getSchedule(e)}}>Thursday</Button>
                    <Button variant="dark" name="friday" onClick={(e)=>{this.getSchedule(e)}}>Friday</Button>
                    <Button variant="dark" name="saturday" onClick={(e)=>{this.getSchedule(e)}}>Saturday</Button>
                    <Button variant="dark" name="sunday" onClick={(e)=>{this.getSchedule(e)}}>Sunday</Button>
                </ButtonGroup>
                <Container style={{border:'4px solid #2e2e2e', borderRadius:'2px', borderTop:'none'}}>
                    {scheduleData[scheduleDay].map((elem, idx) => {return <p key={idx}>{elem.title}</p>})}
                </Container>
            </>
        )
    }
}

export default Schedule;