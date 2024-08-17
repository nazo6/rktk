# rrp - rktk remap protocol

## Spec (draft)

rrp is a protocol to change keyboard config through serial port. This protocol
is transport-agnotic, so you can use any transport layer like USB, BLE, etc.

rrp uses server-client model. Server is a rktk firmware on keyboard and client
is pc or something.

### Endpoint

"Endpoint" is a target of rrp command. There are two type of endpoint.

1. request-response endpoint

   Normal endpoint. Client send command to server and server send response to
   it. This is used to get/set config.

2. request-stream endpoint

   Stream endpoint. Client send command to server and server send stream data to
   it. This is used for endpoint that returns big data to avoid allocation.

### Protocol

The core of this protocol is COBS, which allows framing with little overhead.

Below is how a client makes a request to a server using RRP.

1. Send the endpoint name (COBS-encoded string)

2. Send the request parameters (COBS-encoded value)

   Even if the request parameter is empty (unit type), you must send the
   COBS-encoded value (i.e. [0x01,0x00]).

3. Wait for the COBS-encoded value to be returned (0x00 is returned). In the
   request-stream endpoint, values will continue to be returned after that, but
   if single 0x00 arrives (i.e. if 0x00 is received twice in a row), the stream
   will end.

## Endpoint list

- `get_config` (request-response)
  - Request: `()`
  - Response: `Config`

- `get_keymaps` (request-stream)
  - Request: `()`
  - Response: Stream of `(layer: usize, row: usize, col: usize, key: KeyDef)`
