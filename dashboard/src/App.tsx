import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Dashboard from './pages/Dashboard';
import Settings from './pages/Settings';
import NotFound from './pages/NotFound';
import Header from './components/Header';
import Footer from './components/Footer';

const App = () => {
    return (
        <Router>
            <div className="app-container">
                <Header />
                <Switch>
                    <Route path="/" exact component={Dashboard} />
                    <Route path="/settings" component={Settings} />
                    <Route component={NotFound} />
                </Switch>
                <Footer />
            </div>
        </Router>
    );
};

export default App;