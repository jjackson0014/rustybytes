import { useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Login() {
  const [form, setForm] = useState({ email: "", password: "" });
  const [message, setMessage] = useState("");
  const navigate = useNavigate();

  async function loginUser(e) {
    e.preventDefault();
    
    try {
      const res = await fetch("https://localhost/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(form),
      });

      const data = await res.json();
      console.log("Login Response:", data);

      if (res.ok) {
        localStorage.setItem("token", data.token); // Store JWT Token
        setMessage("Login successful!");
        navigate("/game"); // Redirect to game page
      } else {
        setMessage(`Error: ${data.error || "Invalid credentials"}`);
      }
    } catch (error) {
      console.error("Login error:", error);
      setMessage("Network error");
    }
  }

  return (
    <div>
      <h1>Login</h1>
      <form onSubmit={loginUser}>
        <input
          type="email"
          placeholder="Email"
          value={form.email}
          onChange={(e) => setForm({ ...form, email: e.target.value })}
          required
        />
        <input
          type="password"
          placeholder="Password"
          value={form.password}
          onChange={(e) => setForm({ ...form, password: e.target.value })}
          required
        />
        <button type="submit">Login</button>
      </form>
      <p>{message}</p>
    </div>
  );
}
