import { h, FunctionalComponent } from 'preact';
import { Router, Route } from 'preact-router';
import { Container, Title } from 'preact-bulma';
import { baseroute } from '../baseroute';
import GitHubCorner from './github-corner';
import Home from '../routes/home';
import Result from '../routes/result';

const App: FunctionalComponent = () => {
    return (
        <div id="app">
            <GitHubCorner href="https://github.com/mosmeh/witchbooru" />
            <Container>
                <Title>Witchbooru</Title>
                <Router>
                    <Route path={`${baseroute}/`} component={Home} />
                    <Route path={`${baseroute}/result`} component={Result} />
                </Router>
            </Container>
        </div>
    );
};

export default App;
