import { useState, useEffect } from "react";

export default function useViewportAspectRatio() {
  const getRatio = () => window.innerWidth / window.innerHeight;

  const [aspectRatio, setAspectRatio] = useState(getRatio);

  useEffect(() => {
    const handleResize = () => setAspectRatio(getRatio());
    window.addEventListener("resize", handleResize);

    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return aspectRatio;
}
