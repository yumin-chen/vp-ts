import {
  DieselOrm,
  type JsUser,
  type JsNewUser,
  type JsPost,
  type JsNewPost,
  type JsDownload,
  type JsNewDownload,
} from "../index.js";

export {
  DieselOrm,
  type JsUser,
  type JsNewUser,
  type JsPost,
  type JsNewPost,
  type JsDownload,
  type JsNewDownload,
};

export function establishConnection(databaseUrl: string = ":memory:"): DieselOrm {
  return new DieselOrm(databaseUrl);
}
