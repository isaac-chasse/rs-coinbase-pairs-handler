# WebSocket API JSON Payloads & Formatting

## Subscription Message to the "ticker" Channel
```
{
    "type": "subscribe", 
    "product_ids": [
        "ETH-USD"
        ], 
    "channel": "ticker", 
    "api_key": "KaHjgMpnvTV8rl1r", 
    "timestamp": "1675836190", 
    "signature": "3a576f746dfd8d4c52c890095ae2ec7880b807bc8062ae87c020e88ddbbfad7e"
}
```

## Response Messages from "ticker" Channel

### Error
```
{
	"type": "error",
	"message": "authentication failure"
}
```
### Subscriptions
```
{
	"channel": "subscriptions",
	"client_id": "",
	"timestamp": "2023-02-08T06:12:20.838432067Z",
	"sequence_num": 1,
	"events": [
		{
			"subscriptions": {
				"ticker": [
					"ETH-USD"
				]
			}
		}
	]
}
```
### Snapshot
```
{
	"channel": "ticker",
	"client_id": "",
	"timestamp": "2023-02-08T06:12:20.838410617Z",
	"sequence_num": 0,
	"events": [
		{
			"type": "snapshot",
			"tickers": [
				{
					"type": "ticker",
					"product_id": "ETH-USD",
					"price": "1675.14",
					"volume_24_h": "185976.72526638",
					"low_24_h": "1624.53",
					"high_24_h": "1699.66",
					"low_52_w": "879.8",
					"high_52_w": "3581.6",
					"price_percent_chg_24_h": "2.48889541499945"
				}
			]
		}
	]
}
```
### Update
```
{
	"channel": "ticker",
	"client_id": "",
	"timestamp": "2023-02-08T06:12:21.708730964Z",
	"sequence_num": 2,
	"events": [
		{
			"type": "update",
			"tickers": [
				{
					"type": "ticker",
					"product_id": "ETH-USD",
					"price": "1675.01",
					"volume_24_h": "185976.72526638",
					"low_24_h": "1624.53",
					"high_24_h": "1699.66",
					"low_52_w": "879.8",
					"high_52_w": "3581.6",
					"price_percent_chg_24_h": "2.48094171775388"
				}
			]
		}
	]
}
```