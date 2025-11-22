import { Routes, Route } from "react-router-dom";
import Dashboard from "./pages/Dashboard";
import ListPage from "./pages/ListPage";
import PopupCare from "./pages/components/PopupCare"

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Dashboard />} />
      <Route path="/list" element={<ListPage />}></Route>
      <Route path="/popup/care" element={<PopupCare />}></Route>
    </Routes>
  );
}
