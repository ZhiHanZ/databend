# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: notification.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(
    b'\n\x12notification.proto\x12\x11notificationproto\x1a\x1fgoogle/protobuf/timestamp.proto"@\n\x11NotificationError\x12\x0c\n\x04kind\x18\x01 \x01(\t\x12\x0f\n\x07message\x18\x02 \x01(\t\x12\x0c\n\x04\x63ode\x18\x03 \x01(\x05"\xd9\x02\n\x19\x43reateNotificationRequest\x12\x11\n\ttenant_id\x18\x01 \x01(\t\x12\x15\n\rif_not_exists\x18\x02 \x01(\x08\x12\x0c\n\x04name\x18\x03 \x01(\t\x12>\n\x11notification_type\x18\x04 \x01(\x0e\x32#.notificationproto.NotificationType\x12\x0f\n\x07\x65nabled\x18\x06 \x01(\x08\x12\x13\n\x0bwebhook_url\x18\x07 \x01(\t\x12\x1b\n\x0ewebhook_method\x18\x08 \x01(\tH\x00\x88\x01\x01\x12)\n\x1cwebhook_authorization_header\x18\t \x01(\tH\x01\x88\x01\x01\x12\x15\n\x08\x63omments\x18Z \x01(\tH\x02\x88\x01\x01\x42\x11\n\x0f_webhook_methodB\x1f\n\x1d_webhook_authorization_headerB\x0b\n\t_comments"y\n\x1a\x43reateNotificationResponse\x12\x38\n\x05\x65rror\x18\x01 \x01(\x0b\x32$.notificationproto.NotificationErrorH\x00\x88\x01\x01\x12\x17\n\x0fnotification_id\x18\x02 \x01(\x04\x42\x08\n\x06_error"M\n\x17\x44ropNotificationRequest\x12\x11\n\ttenant_id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x11\n\tif_exists\x18\x03 \x01(\x08"^\n\x18\x44ropNotificationResponse\x12\x38\n\x05\x65rror\x18\x01 \x01(\x0b\x32$.notificationproto.NotificationErrorH\x00\x88\x01\x01\x42\x08\n\x06_error"\xda\x03\n\x0cNotification\x12\x17\n\x0fnotification_id\x18\x01 \x01(\x04\x12\x11\n\ttenant_id\x18\x02 \x01(\t\x12\x0c\n\x04name\x18\x03 \x01(\t\x12>\n\x11notification_type\x18\x04 \x01(\x0e\x32#.notificationproto.NotificationType\x12\x0f\n\x07\x65nabled\x18\x05 \x01(\x08\x12\x13\n\x0bwebhook_url\x18\x06 \x01(\t\x12\x1b\n\x0ewebhook_method\x18\x07 \x01(\tH\x00\x88\x01\x01\x12)\n\x1cwebhook_authorization_header\x18\x08 \x01(\tH\x01\x88\x01\x01\x12\x15\n\x08\x63omments\x18Z \x01(\tH\x02\x88\x01\x01\x12\x30\n\x0c\x63reated_time\x18[ \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12\x12\n\ncreated_by\x18\\ \x01(\t\x12\x30\n\x0cupdated_time\x18] \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12\x12\n\nupdated_by\x18^ \x01(\tB\x11\n\x0f_webhook_methodB\x1f\n\x1d_webhook_authorization_headerB\x0b\n\t_comments",\n\x17ListNotificationRequest\x12\x11\n\ttenant_id\x18\x01 \x01(\t"\x96\x01\n\x18ListNotificationResponse\x12\x38\n\x05\x65rror\x18\x01 \x01(\x0b\x32$.notificationproto.NotificationErrorH\x00\x88\x01\x01\x12\x36\n\rnotifications\x18\x05 \x03(\x0b\x32\x1f.notificationproto.NotificationB\x08\n\x06_error"9\n\x16GetNotificationRequest\x12\x11\n\ttenant_id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t"\x94\x01\n\x17GetNotificationResponse\x12\x38\n\x05\x65rror\x18\x01 \x01(\x0b\x32$.notificationproto.NotificationErrorH\x00\x88\x01\x01\x12\x35\n\x0cnotification\x18\x05 \x01(\x0b\x32\x1f.notificationproto.NotificationB\x08\n\x06_error"\xbf\x02\n\x18\x41lterNotificationRequest\x12\x11\n\ttenant_id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12\x16\n\x0eoperation_type\x18\x03 \x01(\t\x12\x14\n\x07\x65nabled\x18\x04 \x01(\x08H\x00\x88\x01\x01\x12\x18\n\x0bwebhook_url\x18\x05 \x01(\tH\x01\x88\x01\x01\x12\x1b\n\x0ewebhook_method\x18\x06 \x01(\tH\x02\x88\x01\x01\x12)\n\x1cwebhook_authorization_header\x18\x07 \x01(\tH\x03\x88\x01\x01\x12\x15\n\x08\x63omments\x18\x08 \x01(\tH\x04\x88\x01\x01\x42\n\n\x08_enabledB\x0e\n\x0c_webhook_urlB\x11\n\x0f_webhook_methodB\x1f\n\x1d_webhook_authorization_headerB\x0b\n\t_comments"x\n\x19\x41lterNotificationResponse\x12\x38\n\x05\x65rror\x18\x01 \x01(\x0b\x32$.notificationproto.NotificationErrorH\x00\x88\x01\x01\x12\x17\n\x0fnotification_id\x18\x02 \x01(\x04\x42\x08\n\x06_error"\xd9\x01\n\x13NotificationHistory\x12\x30\n\x0c\x63reated_time\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12\x32\n\x0eprocessed_time\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12\x16\n\x0emessage_source\x18\x03 \x01(\t\x12\x0c\n\x04name\x18\x04 \x01(\t\x12\x0f\n\x07message\x18\x05 \x01(\t\x12\x0e\n\x06status\x18\x06 \x01(\t\x12\x15\n\rerror_message\x18\x07 \x01(\t"\xab\x03\n\x1eListNotificationHistoryRequest\x12\x11\n\ttenant_id\x18\x01 \x01(\t\x12\x1e\n\x11notification_name\x18\x02 \x01(\tH\x00\x88\x01\x01\x12\x33\n\nstart_time\x18\x03 \x01(\x0b\x32\x1a.google.protobuf.TimestampH\x01\x88\x01\x01\x12\x31\n\x08\x65nd_time\x18\x04 \x01(\x0b\x32\x1a.google.protobuf.TimestampH\x02\x88\x01\x01\x12\x19\n\x0cresult_limit\x18\x05 \x01(\x05H\x03\x88\x01\x01\x12\x16\n\tpage_size\x18\x06 \x01(\x05H\x04\x88\x01\x01\x12\x1c\n\x0fnext_page_token\x18\x07 \x01(\x03H\x05\x88\x01\x01\x12 \n\x13previous_page_token\x18\x08 \x01(\x03H\x06\x88\x01\x01\x42\x14\n\x12_notification_nameB\r\n\x0b_start_timeB\x0b\n\t_end_timeB\x0f\n\r_result_limitB\x0c\n\n_page_sizeB\x12\n\x10_next_page_tokenB\x16\n\x14_previous_page_token"\xe3\x01\n\x1fListNotificationHistoryResponse\x12\x38\n\x05\x65rror\x18\x01 \x01(\x0b\x32$.notificationproto.NotificationErrorH\x00\x88\x01\x01\x12\x46\n\x16notification_histories\x18\x05 \x03(\x0b\x32&.notificationproto.NotificationHistory\x12\x17\n\x0fnext_page_token\x18\x06 \x01(\x03\x12\x1b\n\x13previous_page_token\x18\x07 \x01(\x03\x42\x08\n\x06_error*\x1f\n\x10NotificationType\x12\x0b\n\x07WEBHOOK\x10\x00\x32\xbf\x05\n\x13NotificationService\x12q\n\x12\x43reateNotification\x12,.notificationproto.CreateNotificationRequest\x1a-.notificationproto.CreateNotificationResponse\x12k\n\x10\x44ropNotification\x12*.notificationproto.DropNotificationRequest\x1a+.notificationproto.DropNotificationResponse\x12k\n\x10ListNotification\x12*.notificationproto.ListNotificationRequest\x1a+.notificationproto.ListNotificationResponse\x12h\n\x0fGetNotification\x12).notificationproto.GetNotificationRequest\x1a*.notificationproto.GetNotificationResponse\x12n\n\x11\x41lterNotification\x12+.notificationproto.AlterNotificationRequest\x1a,.notificationproto.AlterNotificationResponse\x12\x80\x01\n\x17ListNotificationHistory\x12\x31.notificationproto.ListNotificationHistoryRequest\x1a\x32.notificationproto.ListNotificationHistoryResponseB.Z,databend.com/cloudcontrol/notification/protob\x06proto3'
)

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, "notification_pb2", _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b"Z,databend.com/cloudcontrol/notification/proto"
    _globals["_NOTIFICATIONTYPE"]._serialized_start = 2996
    _globals["_NOTIFICATIONTYPE"]._serialized_end = 3027
    _globals["_NOTIFICATIONERROR"]._serialized_start = 74
    _globals["_NOTIFICATIONERROR"]._serialized_end = 138
    _globals["_CREATENOTIFICATIONREQUEST"]._serialized_start = 141
    _globals["_CREATENOTIFICATIONREQUEST"]._serialized_end = 486
    _globals["_CREATENOTIFICATIONRESPONSE"]._serialized_start = 488
    _globals["_CREATENOTIFICATIONRESPONSE"]._serialized_end = 609
    _globals["_DROPNOTIFICATIONREQUEST"]._serialized_start = 611
    _globals["_DROPNOTIFICATIONREQUEST"]._serialized_end = 688
    _globals["_DROPNOTIFICATIONRESPONSE"]._serialized_start = 690
    _globals["_DROPNOTIFICATIONRESPONSE"]._serialized_end = 784
    _globals["_NOTIFICATION"]._serialized_start = 787
    _globals["_NOTIFICATION"]._serialized_end = 1261
    _globals["_LISTNOTIFICATIONREQUEST"]._serialized_start = 1263
    _globals["_LISTNOTIFICATIONREQUEST"]._serialized_end = 1307
    _globals["_LISTNOTIFICATIONRESPONSE"]._serialized_start = 1310
    _globals["_LISTNOTIFICATIONRESPONSE"]._serialized_end = 1460
    _globals["_GETNOTIFICATIONREQUEST"]._serialized_start = 1462
    _globals["_GETNOTIFICATIONREQUEST"]._serialized_end = 1519
    _globals["_GETNOTIFICATIONRESPONSE"]._serialized_start = 1522
    _globals["_GETNOTIFICATIONRESPONSE"]._serialized_end = 1670
    _globals["_ALTERNOTIFICATIONREQUEST"]._serialized_start = 1673
    _globals["_ALTERNOTIFICATIONREQUEST"]._serialized_end = 1992
    _globals["_ALTERNOTIFICATIONRESPONSE"]._serialized_start = 1994
    _globals["_ALTERNOTIFICATIONRESPONSE"]._serialized_end = 2114
    _globals["_NOTIFICATIONHISTORY"]._serialized_start = 2117
    _globals["_NOTIFICATIONHISTORY"]._serialized_end = 2334
    _globals["_LISTNOTIFICATIONHISTORYREQUEST"]._serialized_start = 2337
    _globals["_LISTNOTIFICATIONHISTORYREQUEST"]._serialized_end = 2764
    _globals["_LISTNOTIFICATIONHISTORYRESPONSE"]._serialized_start = 2767
    _globals["_LISTNOTIFICATIONHISTORYRESPONSE"]._serialized_end = 2994
    _globals["_NOTIFICATIONSERVICE"]._serialized_start = 3030
    _globals["_NOTIFICATIONSERVICE"]._serialized_end = 3733
# @@protoc_insertion_point(module_scope)
