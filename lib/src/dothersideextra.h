/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#ifndef DOTHERSIDEEXTRA_H
#define DOTHERSIDEEXTRA_H

#include "detypes.h"

#ifdef __cplusplus
extern "C" {
#endif

DEApplication *de_qguiapplication_create(int argc, const char *const *argv);
void de_qguiapplication_delete(DEApplication *vptr);

DosQQuickView *de_qquickview_create();
void de_qquickview_set_source_url(DosQQuickView *vptr, const DosQUrl *url);

DEObject *de_qobject_create(const DosQMetaObject *metaObject, DObjectCallback dObjectCallback);
void de_qobject_set_dobject(DEObject *vptr, void *dObject);
void *de_qobject_check_and_get_dobject(DEObject *vptr, const DosQMetaObject *meta);

DosQMetaObject *de_qlistmodel_qmetaobject();
DEListModel *de_qlistmodel_create(const DosQMetaObject *metaObject, const char *const *roleArray, int roleArrayLength,
                                  DObjectCallback dObjectCallback);
void de_qlistmodel_set_dobject(DEListModel *vptr, void *dObject);
int de_qlistmodel_count(const DEListModel *vptr);
bool de_qlistmodel_empty(const DEListModel *vptr);
void de_qlistmodel_insert(DEListModel *vptr, int row, const DEQvariantMapList *values);
void de_qlistmodel_remove(DEListModel *vptr, int row, int count);
DEQVariantMap *de_qlistmodel_get(const DEListModel *vptr, int index);

int de_qqml_qmlregisterobject(const QmlRegisterType *qmlRegisterType);

DosQVariant *de_qvariant_create_qvariantmap(const DEQVariantMap *value);
DEQVariantMap *de_qvariant_to_qvariantmap(const DosQVariant *vptr);
void de_qvariantmap_delete(const DEQVariantMap *vptr);

#ifdef __cplusplus
}
#endif

#endif // DOTHERSIDE_H
