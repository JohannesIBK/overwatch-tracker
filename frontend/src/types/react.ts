import React from "react";

export type UseState<T> = React.Dispatch<React.SetStateAction<T>>;
export type UseRef<T> = React.MutableRefObject<T>;
