// Node 18+ đã có fetch built-in. Nếu dùng Node cũ hơn:
// npm install node-fetch
// import fetch from "node-fetch";

const CONCURRENCY = 100;      // số "luồng" (request concurrent)
const TOTAL_REQUESTS = 50000000; // tổng số request muốn bắn
const URL = "http://0.0.0.0:8080/api/v1/user/28";

// chuẩn bị header
const myHeaders = new Headers();
myHeaders.append(
  "Authorization",
  "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3NjMwOTQwNzgsImV4cCI6MTc2MzY5ODg3OCwibmJmIjoxNzYzMDk0MDc4LCJzdWIiOiIyOCJ9.Ql3Yofsf8x0oB9II_qRuKdG3UlaxiW4uLHo9px4AaqA"
);

const requestOptions = {
  method: "GET",
  headers: myHeaders,
  redirect: "follow",
};
let count=0;

async function worker(id, count) {
  let success = 0;
  let fail = 0;

  for (let i = 0; i < count; i++) {
    try {
      const res = await fetch(URL, requestOptions);
      console.log(`Worker ${id} - Request ${++count}: ${res.status}`);
      if (res.ok) {
        success++;
        // nếu muốn đọc body thì:
        // const text = await res.text();
      } else {
        fail++;
        console.error(`Worker ${id}: status ${res.status}`);
      }
    } catch (e) {
      fail++;
      console.error(`Worker ${id}: error`, e.message);
    }
  }

  return { success, fail };
}
async function main() {
  const perWorker = Math.floor(TOTAL_REQUESTS / CONCURRENCY);
  const extra = TOTAL_REQUESTS % CONCURRENCY;

  console.log(`Bắn ${TOTAL_REQUESTS} request với ${CONCURRENCY} luồng...`);

  const start = Date.now();

  const promises = [];
  for (let i = 0; i < CONCURRENCY; i++) {
    const thisCount = perWorker + (i < extra ? 1 : 0); // chia đều phần dư
    promises.push(worker(i, thisCount));
  }
  const results = await Promise.all(promises);
  const end = Date.now();

  const totalSuccess = results.reduce((sum, r) => sum + r.success, 0);
  const totalFail = results.reduce((sum, r) => sum + r.fail, 0);
  const durationSec = (end - start) / 1000;
  const rps = totalSuccess / durationSec;

  console.log("------ Kết quả ------");
  console.log("Thời gian:", durationSec.toFixed(2), "s");
  console.log("Thành công:", totalSuccess);
  console.log("Lỗi:", totalFail);
  console.log("Throughput ~", rps.toFixed(2), "req/s");
}

main().catch(console.error);
