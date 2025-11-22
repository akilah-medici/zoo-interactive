import { Routes, Route } from "react-router-dom";
import Dashboard from "./pages/Dashboard";
import ListPage from "./pages/ListPage";
import ModifyPage from "./pages/ModifyPage";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Dashboard />} />
      <Route path="/list" element={<ListPage />}></Route>
      <Route path="/modify" element={<ModifyPage />}></Route>
    </Routes>
  );
}
