import { useState } from "react";

export const useModal = () => {
  const [showModal, setShowModal] = useState(false);
  const toggleModal = () => setShowModal((prev) => !prev);
  const closeModal = () => setShowModal(false);
  const openModal = () => setShowModal(true);

  return {
    showModal,
    setShowModal,
    toggleModal,
    closeModal,
    openModal,
  };
};
