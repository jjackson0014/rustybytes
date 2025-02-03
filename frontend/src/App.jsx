import { BrowserRouter as Router, Routes, Route, Link, useNavigate } from "react-router-dom";
import Register from "./pages/Register";
import Login from "./pages/Login";
import Game from "./pages/Game";
import ProtectedRoute from "./components/ProtectedRoute";
import './App.css';

function App() {
  const token = localStorage.getItem("token");
  const navigate = useNavigate();

  function logout() {
    localStorage.removeItem("token");
    navigate("/login");
  }

  return (
    <>
      <nav>
        <Link to="/">Home</Link>
        {token ? (
          <>
            <Link to="/game">Game</Link>
            <button onClick={logout}>Logout</button>
          </>
        ) : (
          <>
            <Link to="/register">Register</Link>
            <Link to="/login">Login</Link>
          </>
        )}
      </nav>
      <Routes>
        <Route path="/" element={<h1>Welcome to RustyBytes!</h1>} />
        <Route path="/register" element={<Register />} />
        <Route path="/login" element={<Login />} />
        <Route path="/game" element={<ProtectedRoute><Game /></ProtectedRoute>} />
      </Routes>
    </>
  );
}

export default App;
